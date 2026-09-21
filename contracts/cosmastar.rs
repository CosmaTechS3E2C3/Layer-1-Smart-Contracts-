//! CosmaStar – domain-specific contract stub.

use serde_json::json;
use crate::engine::abi::ContractCall;
use crate::engine::contract_vm::ContractVm;

pub fn handle(_vm: &mut ContractVm, call: ContractCall) -> serde_json::Value {
    json!({ "contract": "cosmastar", "method": call.method, "args": call.args })
}

