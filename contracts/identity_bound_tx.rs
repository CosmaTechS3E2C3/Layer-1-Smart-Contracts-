//! Identity-bound transactions – tie actions to identity.

use serde_json::json;
use crate::engine::abi::ContractCall;
use crate::engine::contract_vm::ContractVm;

pub fn handle(vm: &mut ContractVm, call: ContractCall) -> serde_json::Value {
    match call.method.as_str() {
        "record_tx" => {
            let tx_id = call.args["tx_id"].as_str().unwrap_or("unknown");
            let identity = &call.sender;
            vm.state.set(&format!("idtx:{}:{}", identity, tx_id), call.args.to_string());
            json!({ "status": "ok", "tx_id": tx_id, "identity": identity })
        }
        "get_tx" => {
            let tx_id = call.args["tx_id"].as_str().unwrap_or("unknown");
            let identity = call.args["identity"].as_str().unwrap_or("unknown");
            let data = vm.state.get(&format!("idtx:{}:{}", identity, tx_id));
            json!({ "tx_id": tx_id, "identity": identity, "data": data })
        }
        _ => json!({ "error": "unknown_method" }),
    }
}

