use crate::core::tx::tx_types::{Transaction, TxType};
use crate::core::tx::tx_pool::TxPool;
use crate::core::state::state_manager::StateManager;
use crate::core::bindings::l0_bridge::L0Bridge;

#[derive(Debug)]
pub struct CosmaCoreClient<'a> {
    pub state: &'a mut StateManager,
    pub tx_pool: &'a mut TxPool,
}

impl<'a> CosmaCoreClient<'a> {
    pub fn submit_tx(&mut self, tx_type: TxType, sender: &str, payload: Vec<u8>) -> bool {
        let tx = Transaction::new(tx_type, sender, payload);
        self.tx_pool.submit_tx(tx)
    }

    pub fn get_balance(&self, user: &str) -> u64 {
        self.state.accounts.get_account(user)
            .map(|acc| acc.balance)
            .unwrap_or(0)
    }

    pub fn get_scs_phase(&self, user: &str) -> String {
        format!("Phase for {}", user) // placeholder until SCS registry is added
    }

    pub fn anchor_state_root(&self) {
        let root = self.state.compute_state_root();
        let _ = L0Bridge::anchor(&root);
    }
}

