//! Contract state – simple key/value store per contract.

use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Default, Clone)]
pub struct ContractState {
    pub kv: HashMap<String, String>,
}

impl ContractState {
    pub fn new() -> Self {
        Self { kv: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.kv.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.kv.get(key).cloned()
    }

    pub fn compute_state_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for (k, v) in &self.kv {
            hasher.update(k.as_bytes());
            hasher.update(v.as_bytes());
        }
        let bytes = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(&bytes[..32]);
        root
    }
}

