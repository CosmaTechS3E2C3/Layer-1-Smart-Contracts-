use axum::{Router, routing::post, routing::get, Json};
use crate::core::tx::tx_pool::TxPool;
use crate::core::tx::tx_types::{Transaction, TxType};
use crate::core::state::state_manager::StateManager;

async fn submit_contract(Json(payload): Json<String>) -> Json<bool> {
    let mut pool = TxPool::new();
    let ok = pool.submit_tx(
        Transaction::new(TxType::SCS, "app", payload.into_bytes())
    );
    Json(ok)
}

async fn scs_phase(user: String) -> Json<String> {
    Json(format!("Phase for {}", user))
}

async fn balance(user: String) -> Json<u64> {
    let state = StateManager::new();
    Json(
        state.accounts.get_account(&user)
            .map(|acc| acc.balance)
            .unwrap_or(0)
    )
}

pub fn build_contract_router() -> Router {
    Router::new()
        .route("/contract/submit", post(submit_contract))
        .route("/contract/scs/:user", get(|params| async move {
            scs_phase(params).await
        }))
        .route("/contract/balance/:user", get(|params| async move {
            balance(params).await
        }))
}
