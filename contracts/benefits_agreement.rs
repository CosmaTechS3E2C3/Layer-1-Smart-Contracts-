use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::types::contract_event::{ContractEvent, emit_event};

pub fn benefits_agreement(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 2 {
        return "Invalid args".to_string();
    }

    let user = &args[0];
    let benefit = &args[1];

    state.set(&format!("benefit_{}", user), benefit);

    emit_event(&ContractEvent::LoyaltyReward {
        user: user.clone(),
        points: 50,
    });

    format!("Benefit {} applied to {}", benefit, user)
}
