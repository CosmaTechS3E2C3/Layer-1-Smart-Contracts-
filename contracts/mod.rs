pub mod benefits_agreement;
pub mod cosmastar;
pub mod cosmcare;
pub mod filmcore;
pub mod identity_bound_tx;
pub mod revenue_split;
pub mod cosmagigs;
pub mod caddiepro;
pub mod cosmasocial;

use crate::core::smart_contracts::engine::contract::Contract;

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

