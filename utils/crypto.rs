use sha2::{Sha256, Digest};

pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn hash_to_hex(data: &[u8]) -> String {
    let bytes = hash(data);
    hex::encode(bytes)
}


