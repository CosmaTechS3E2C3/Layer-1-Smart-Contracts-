// Layer‑1 Smart‑Contracts Root Library

// -------------------------------
// Engine Modules
// -------------------------------
pub mod engine {
    pub mod abi;
    pub mod contract;
    pub mod contract_state;
    pub mod contract_vm;
    pub mod dispatcher;
}

// -------------------------------
// Type System
// -------------------------------
pub mod types {
    pub mod contract_event;
    pub mod cosmacore_client;
    pub mod contract; // type-level contract definition
}

// -------------------------------
// Domain Contracts
// -------------------------------
pub mod contracts {
    pub mod benefits_agreement;
    pub mod cosmastar;
    pub mod cosmcare;
    pub mod filmcore;
    pub mod identity_bound_tx;
    pub mod revenue_split;
    pub mod cosmagigs;
    pub mod caddiepro;
    pub mod cosmasocial;

    // Registry loader
    pub mod registry {
        use crate::engine::contract::Contract;

        use super::{
            benefits_agreement,
            cosmastar,
            cosmcare,
            filmcore,
            identity_bound_tx,
            revenue_split,
            cosmagigs,
            caddiepro,
            cosmasocial,
        };

        pub fn load_all_contracts() -> Vec<Contract> {
            vec![
                Contract::new("benefits", "BenefitsAgreement", "1.0", benefits_agreement::benefits_agreement),
                Contract::new("cosmastar", "CosmaStar", "1.0", cosmastar::cosmastar),
                Contract::new("cosmcare", "CosmaCare", "1.0", cosmcare::cosmcare),
                Contract::new("filmcore", "FilmCore", "1.0", filmcore::filmcore),
                Contract::new("identitytx", "IdentityBoundTx", "1.0", identity_bound_tx::identity_bound_tx),
                Contract::new("revsplit", "RevenueSplit", "1.0", revenue_split::revenue_split),
                Contract::new("cosmagigs", "CosmaGigs", "1.0", cosmagigs::cosmagigs),
                Contract::new("caddiepro", "CaddiePro", "1.0", caddiepro::caddiepro),
                Contract::new("cosmasocial", "CosmaSocial", "1.0", cosmasocial::cosmasocial),
            ]
        }
    }
}

// -------------------------------
// Public Re‑Exports
// -------------------------------

// Engine
pub use engine::abi::*;
pub use engine::contract::*;
pub use engine::contract_state::*;
pub use engine::contract_vm::*;
pub use engine::dispatcher::*;

// Types
pub use types::contract_event::*;
pub use types::cosmacore_client::*;
pub use types::contract::*;

// Contracts
pub use contracts::registry::load_all_contracts;

