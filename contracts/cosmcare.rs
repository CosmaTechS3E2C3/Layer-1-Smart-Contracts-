use crate::core::smart_contracts::engine::contract_state::ContractState;
use crate::core::smart_contracts::types::contract_event::{ContractEvent, emit_event};

pub fn cosmcare(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 2 {
        return "Invalid args".to_string();
    }

    let user = &args[0];
    let care_type = &args[1];

    state.set(&format!("care_{}", user), care_type);

    emit_event(&ContractEvent::SCSPhaseChanged {
        user: user.clone(),
        phase: "CareUpdated".to_string(),
    });

    format!("CosmaCare updated for {}: {}", user, care_type)
}
