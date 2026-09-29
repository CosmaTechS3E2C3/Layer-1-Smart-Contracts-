#[derive(Debug, Clone)]
pub enum ContractEvent {
    SCSPhaseChanged {
        user: String,
        phase: String,
    },
    BalanceUpdated {
        user: String,
        new_balance: u64,
    },
    LoyaltyReward {
        user: String,
        points: u64,
    },
    IdentityVerified {
        user: String,
    },
}

pub fn emit_event(event: &ContractEvent) {
    println!("[Contract Event] {:?}", event);
}

