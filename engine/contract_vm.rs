//! Contract VM – executes contract calls against contract state.

use crate::engine::abi::ContractCall;
use crate::storage::contract_state::ContractState;

pub struct ContractVm {
    pub state: ContractState,
}

impl ContractVm {
    pub fn new(state: ContractState) -> Self {
        Self { state }
    }

    pub fn execute(&mut self, call: ContractCall) -> serde_json::Value {
        // Dispatch to specific contract module
        crate::engine::dispatcher::dispatch(self, call)
    }
}

