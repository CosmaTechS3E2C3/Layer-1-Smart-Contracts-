//! Benefits Agreement – SCS-aligned benefit contracts.

use serde_json::json;
use crate::engine::abi::ContractCall;
use crate::engine::contract_vm::ContractVm;

pub fn handle(vm: &mut ContractVm, call: ContractCall) -> serde_json::Value {
    match call.method.as_str() {
        "create_agreement" => {
            let id = call.args["agreement_id"].as_str().unwrap_or("unknown");
            vm.state.set(&format!("benefits:{}", id), call.args.to_string());
            json!({ "status": "ok", "agreement_id": id })
        }
        "get_agreement" => {
            let id = call.args["agreement_id"].as_str().unwrap_or("unknown");
            let data = vm.state.get(&format!("benefits:{}", id));
            json!({ "agreement_id": id, "data": data })
        }
        _ => json!({ "error": "unknown_method" }),
    }
}

