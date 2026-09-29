use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::types::contract_event::{ContractEvent, emit_event};

pub fn identity_bound_tx(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 2 {
        return "Invalid args".to_string();
    }

    let user = &args[0];
    let tx_data = &args[1];

    state.set(&format!("idtx_{}", user), tx_data);

    emit_event(&ContractEvent::IdentityVerified {
        user: user.clone(),
    });

    format!("Identity-bound transaction recorded for {}", user)
}
