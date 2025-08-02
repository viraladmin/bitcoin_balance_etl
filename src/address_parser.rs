use bitcoin::{Address, Network, ScriptBuf};
use bitcoin::hashes::{sha256, Hash};

pub fn parse_address(script: &ScriptBuf, network: Network) -> String {

    match Address::from_script(script, network) {
        Ok(address) => address.to_string(),
        Err(_) => {
            let script_hash = sha256::Hash::hash(script.as_bytes());
            format!("script:{}", script_hash)
        }
    }
}
