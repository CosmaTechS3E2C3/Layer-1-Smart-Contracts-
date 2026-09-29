use serde::{Serialize, Deserialize};

pub fn to_json<T: Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

pub fn from_json<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Option<T> {
    serde_json::from_slice(bytes).ok()
}

pub fn encode_string_args(args: &[String]) -> Vec<u8> {
    to_json(args)
}

pub fn decode_string_args(bytes: &[u8]) -> Option<Vec<String>> {
    from_json(bytes)
}


