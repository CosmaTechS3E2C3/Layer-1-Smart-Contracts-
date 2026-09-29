use crate::core::smart_contracts::engine::contract_state::ContractState;

#[derive(Debug, Clone)]
pub struct Contract {
    pub address: String,
    pub name: String,
    pub version: String,
    pub code: fn(&mut ContractState, Vec<String>) -> String,
}

impl Contract {
    pub fn new(
        address: &str,
        name: &str,
        version: &str,
        code: fn(&mut ContractState, Vec<String>) -> String
    ) -> Self {
        Self {
            address: address.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            code,
        }
    }

    pub fn execute(&self, state: &mut ContractState, args: Vec<String>) -> String {
        (self.code)(state, args)
    }
}

