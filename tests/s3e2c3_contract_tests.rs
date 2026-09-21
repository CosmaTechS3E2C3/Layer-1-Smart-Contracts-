//! Tests for S3E2C3-aligned contracts (benefits, identity-bound, revenue).

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::contract_vm::ContractVm;
    use crate::engine::abi::ContractCall;
    use crate::storage::contract_state::ContractState;
    use serde_json::json;

    #[test]
    fn create_and_get_benefits_agreement() {
        let state = ContractState::new();
        let mut vm = ContractVm::new(state);

        let call = ContractCall {
            contract: "benefits_agreement".into(),
            method: "create_agreement".into(),
            args: json!({ "agreement_id": "a1", "terms": "test" }),
            sender: "alice".into(),
        };

        let res = vm.execute(call);
        assert_eq!(res["status"], "ok");

        let get_call = ContractCall {
            contract: "benefits_agreement".into(),
            method: "get_agreement".into(),
            args: json!({ "agreement_id": "a1" }),
            sender: "alice".into(),
        };

        let res2 = vm.execute(get_call);
        assert_eq!(res2["agreement_id"], "a1");
    }
}

