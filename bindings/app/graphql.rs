use async_graphql::{Object, Schema, SimpleObject};
use crate::core::tx::tx_types::{TxType};
use crate::core::tx::tx_pool::TxPool;
use crate::core::state::state_manager::StateManager;

pub struct ContractQuery;

#[Object]
impl ContractQuery {
    async fn scs_phase(&self, user: String) -> String {
        format!("Phase for {}", user)
    }

    async fn balance(&self, user: String) -> u64 {
        let state = StateManager::new();
        state.accounts.get_account(&user)
            .map(|acc| acc.balance)
            .unwrap_or(0)
    }
}

pub struct ContractMutation;

#[Object]
impl ContractMutation {
    async fn submit_contract_tx(&self, sender: String, payload: String) -> bool {
        let mut pool = TxPool::new();
        pool.submit_tx(
            crate::core::tx::tx_types::Transaction::new(
                TxType::SCS,
                &sender,
                payload.into_bytes()
            )
        )
    }
}

pub type ContractSchema = Schema<ContractQuery, ContractMutation, async_graphql::EmptySubscription>;
