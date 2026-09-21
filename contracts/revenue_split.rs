//! Revenue Split – splits flows between parties.

use serde_json::json;
use crate::engine::abi::ContractCall;
use crate::engine::contract_vm::ContractVm;

pub fn handle(vm: &mut ContractVm, call: ContractCall) -> serde_json::Value {
    match call.method.as_str() {
        "define_split" => {
            let id = call.args["split_id"].as_str().unwrap_or("unknown");
            vm.state.set(&format!("split:{}", id), call.args.to_string());
            json!({ "status": "ok", "split_id": id })
        }
        "get_split" => {
            let id = call.args["split_id"].as_str().unwrap_or("unknown");
            let data = vm.state.get(&format!("split:{}", id));
            json!({ "split_id": id, "data": data })
        }
        _ => json!({ "error": "unknown_method" }),
    }
}

