//! REST API – expose contract engine to PWA / Expo / Supabase.

use axum::{Router, routing::post, Json};
use crate::engine::contract_vm::ContractVm;
use crate::engine::abi::ContractCall;
use crate::storage::contract_state::ContractState;

pub fn router() -> Router {
    Router::new().route("/contract/call", post(handle_call))
}

async fn handle_call(Json(call): Json<ContractCall>) -> Json<serde_json::Value> {
    let state = ContractState::new();
    let mut vm = ContractVm::new(state);
    let result = vm.execute(call);
    Json(result)
}

