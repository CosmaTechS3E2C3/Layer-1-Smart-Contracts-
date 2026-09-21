//! ABI – simple JSON-based contract call format.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCall {
    pub contract: String,
    pub method: String,
    pub args: serde_json::Value,
    pub sender: String,
}

