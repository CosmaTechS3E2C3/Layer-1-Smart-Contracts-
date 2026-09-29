use std::collections::HashMap;

use crate::core::smart_contracts::engine::contract::Contract;
use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::engine::contract_vm::ContractVM;

#[derive(Debug)]
pub struct Dispatcher {
    pub registry: HashMap<String, Contract>,
    pub states: HashMap<String, ContractState>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn register(&mut self, contract: Contract) {
        self.states.insert(contract.address.clone(), ContractState::new());
        self.registry.insert(contract.address.clone(), contract);
    }

    pub fn dispatch(&mut self, address: &str, payload: &[u8]) -> String {
        let contract = match self.registry.get(address) {
            Some(c) => c,
            None => return "Contract not found".to_string(),
        };

        let state = self.states.get_mut(address).unwrap();
        let vm = ContractVM::new();

        vm.run(contract, state, payload)
    }
}

