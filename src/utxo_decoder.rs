use bitcoin::ScriptBuf;
use bitcoin::PublicKey;
use crate::numbers::{ decode_varint, decompress_amount };

pub fn decode_utxo_key(mut key: &[u8]) -> Option<(Vec<u8>, u32)> {
    if key.is_empty() || key[0] != b'C' {
        return None;
    }
    key = &key[1..];
    let txid = key.get(..32)?.to_vec();
    key = &key[32..];
    let (vout, _) = decode_varint(key)?;
    Some((txid, vout as u32))
}

fn decompress_script(data: &[u8]) -> Option<(ScriptBuf, usize)> {
    if data.is_empty() {
        return None;
    }

    let tag = data[0];
    let mut i = 1;


    let script = match tag {
        // 0x00: P2PKH
        0x00 => {
            if data.len() < i + 20 {
                return None;
            }
            let mut v = Vec::with_capacity(25);
            v.push(0x76); // OP_DUP
            v.push(0xa9); // OP_HASH160
            v.push(0x14); // push 20
            v.extend_from_slice(&data[i..i + 20]);
            v.push(0x88); // OP_EQUALVERIFY
            v.push(0xac); // OP_CHECKSIG
            i += 20;
            v
        }

        // 0x01: P2SH
        0x01 => {
            if data.len() < i + 20 {
                return None;
            }
            let mut v = Vec::with_capacity(23);
            v.push(0xa9); // OP_HASH160
            v.push(0x14); // push 20
            v.extend_from_slice(&data[i..i + 20]);
            v.push(0x87); // OP_EQUAL
            i += 20;
            v
        }

        // 0x02 or 0x03: P2PK with compressed pubkey
        0x02 | 0x03 => {
            if data.len() < i + 32 {
                return None;
            }
            let mut v = Vec::with_capacity(35);
            v.push(0x21); // push 33
            v.push(tag);  // 0x02 or 0x03
            v.extend_from_slice(&data[i..i + 32]);
            v.push(0xac); // OP_CHECKSIG
            i += 32;
            v
        }

        // 0x04 or 0x05: uncompressed P2PK (decompress from 32-byte + parity)
        // We skip actual decompression; just note it here in case needed later
        0x04 | 0x05 => {
            if data.len() < i + 32 {
                return None;
            }

            let parity = if tag == 0x04 { 0x02 } else { 0x03 };
            let x_bytes = &data[i..i+32];
            i += 32;

            let mut compressed = Vec::with_capacity(33);
            compressed.push(parity);
            compressed.extend_from_slice(x_bytes);
     
            let compressed_pk = PublicKey::from_slice(&compressed).ok()?;
            let uncompressed_ser = compressed_pk.inner.serialize_uncompressed(); // ✅ correct and sufficient

            let mut v = Vec::with_capacity(67);
            v.push(0x41); // push 65
            v.extend_from_slice(&uncompressed_ser);
            v.push(0xac); // OP_CHECKSIG
            v
        }

        // >= 0x06: raw script, next is varint length + script bytes
        _ => {
            let v = data[1..].to_vec();
            i = data.len(); // consume all
            v
        }
    };

    Some((ScriptBuf::from_bytes(script), i))
}


pub fn decode_utxo_value(data: &[u8]) -> Option<(i64, ScriptBuf)> {
    // 1. Decode height + coinbase flag
    let (_, header_len) = decode_varint(&data)?;

    let after_header = &data[header_len..];

    // 2. Decode compressed amount
    let (compressed_amount, amount_len) = decode_varint(&after_header)?;
    let amount = decompress_amount(compressed_amount as i64);
    let after_amount = &after_header[amount_len..];

    // 3. Try decompress_script first
    let script_result = decompress_script(after_amount);

    let (script, _script_len) = match script_result {
        Some((s, l)) => {
            (s, l)
        },
        None => {
            let (script_len, len_len) = decode_varint(after_amount)?;
            let total_len = len_len + script_len as usize;

            if after_amount.len() < total_len {
                return None;
            }

            let raw_script = after_amount[len_len..total_len].to_vec();
            (ScriptBuf::from_bytes(raw_script), total_len)
        }
    };
    Some((amount, script))
}

