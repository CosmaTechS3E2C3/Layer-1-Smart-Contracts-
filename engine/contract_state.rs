use std::collections::HashMap;

#[derive(Debug)]
pub struct ContractState {
    pub storage: HashMap<String, String>,
}

impl ContractState {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.get(key).cloned()
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.storage.insert(key.to_string(), value.to_string());
    }
}



