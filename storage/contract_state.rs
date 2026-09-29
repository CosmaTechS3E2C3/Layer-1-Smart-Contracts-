use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::core::smart_contracts::engine::contract_state::ContractState;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistentContractState {
    pub storage: HashMap<String, String>,
}

impl PersistentContractState {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn from_runtime(state: &ContractState) -> Self {
        Self {
            storage: state.storage.clone(),
        }
    }

    pub fn to_runtime(&self) -> ContractState {
        ContractState {
            storage: self.storage.clone(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
    }
}

/// Global contract storage map (placeholder until integrated with StateManager/DB)
#[derive(Debug, Default)]
pub struct ContractStorageRegistry {
    pub contracts: HashMap<String, PersistentContractState>,
}

impl ContractStorageRegistry {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    pub fn save(&mut self, address: &str, state: &ContractState) {
        let persistent = PersistentContractState::from_runtime(state);
        self.contracts.insert(address.to_string(), persistent);
    }

    pub fn load(&self, address: &str) -> Option<ContractState> {
        self.contracts.get(address).map(|p| p.to_runtime())
    }
}


