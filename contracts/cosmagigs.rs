use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::types::contract_event::{ContractEvent, emit_event};

pub fn cosmagigs(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 3 {
        return "Invalid args".to_string();
    }

    let worker = &args[0];
    let gig_id = &args[1];
    let amount = args[2].parse::<u64>().unwrap_or(0);

    // Store payout
    state.set(&format!("gigpay_{}_{}", worker, gig_id), &amount.to_string());

    // Reward loyalty points
    emit_event(&ContractEvent::LoyaltyReward {
        user: worker.clone(),
        points: 15,
    });

    format!("CosmaGigs payout {} assigned to {} for gig {}", amount, worker, gig_id)
}

