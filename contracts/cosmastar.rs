use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::types::contract_event::{ContractEvent, emit_event};

pub fn cosmastar(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 2 {
        return "Invalid args".to_string();
    }

    let user = &args[0];
    let boost = &args[1];

    state.set(&format!("visibility_{}", user), boost);

    emit_event(&ContractEvent::LoyaltyReward {
        user: user.clone(),
        points: 25,
    });

    format!("CosmaStar visibility boosted for {} by {}", user, boost)
}
