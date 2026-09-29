use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AbiCall {
    pub function: String,
    pub args: Vec<String>,
}

impl AbiCall {
    pub fn decode(payload: &[u8]) -> Option<Self> {
        serde_json::from_slice(payload).ok()
    }

    pub fn encode(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

pub fn selector(function: &str) -> u32 {
    // simple selector: hash first 4 bytes
    let hash = crate::core::utils::crypto::hash(function.as_bytes());
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}
