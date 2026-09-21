//! Tests for domain contracts (CosmaStar, CosmaCare, FilmCore).

#[cfg(test)]
mod tests {
    use crate::engine::contract_vm::ContractVm;
    use crate::engine::abi::ContractCall;
    use crate::storage::contract_state::ContractState;
    use serde_json::json;

    #[test]
    fn cosmastar_stub_works() {
        let state = ContractState::new();
        let mut vm = ContractVm::new(state);

        let call = ContractCall {
            contract: "cosmastar".into(),
            method: "demo".into(),
            args: json!({ "x": 1 }),
            sender: "alice".into(),
        };

        let res = vm.execute(call);
        assert_eq!(res["contract"], "cosmastar");
    }
}

