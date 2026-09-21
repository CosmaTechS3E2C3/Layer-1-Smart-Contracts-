//! Dispatcher – routes calls to specific contract modules.

use crate::engine::abi::ContractCall;
use crate::storage::contract_state::ContractState;
use crate::contracts::{
    benefits_agreement,
    revenue_split,
    identity_bound_tx,
    cosmastar,
    cosmcare,
    filmcore,
};

use serde_json::json;

pub fn dispatch(vm: &mut crate::engine::contract_vm::ContractVm, call: ContractCall) -> serde_json::Value {
    match call.contract.as_str() {
        "benefits_agreement" => benefits_agreement::handle(vm, call),
        "revenue_split"      => revenue_split::handle(vm, call),
        "identity_bound_tx"  => identity_bound_tx::handle(vm, call),
        "cosmastar"          => cosmastar::handle(vm, call),
        "cosmcare"           => cosmcare::handle(vm, call),
        "filmcore"           => filmcore::handle(vm, call),
        _ => json!({ "error": "unknown_contract" }),
    }
}

