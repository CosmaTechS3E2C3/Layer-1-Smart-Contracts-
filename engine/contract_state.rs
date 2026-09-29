use crate::core::smart_contracts::engine::abi::AbiCall;
use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::engine::contract::Contract;

#[derive(Debug)]
pub struct ContractVM;

impl ContractVM {
    pub fn new() -> Self {
        ContractVM
    }

    pub fn run(&self, contract: &Contract, state: &mut ContractState, payload: &[u8]) -> String {
        let call = match AbiCall::decode(payload) {
            Some(c) => c,
            None => return "ABI decode failed".to_string(),
        };

        contract.execute(state, call.args)
    }
}


