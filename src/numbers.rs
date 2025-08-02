

pub fn decode_varint(data: &[u8]) -> Option<(i64, usize)> {
    let mut n: i64 = 0;
    for (i, &byte) in data.iter().enumerate().take(9) {
        n = n << 7;
        n |= (byte & 0x7F) as i64;
        if byte & 0x80 != 0 {
            n += 1;
        } else {
            return Some((n, i + 1));
        }
    }
    None
}

pub fn decompress_amount(x: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    let mut x = x - 1;
    let e = x % 10;
    x /= 10;
    let mut n;
    if e < 9 {
        let d = x % 9;
        x /= 9;
        n = x * 10 + d + 1;
    } else {
        n = x + 1;
    }
    for _ in 0..e {
        n *= 10;
    }
    n
}
