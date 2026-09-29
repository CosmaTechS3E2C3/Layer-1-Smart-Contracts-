use cosmachain_smart_contracts::engine::contract_state::ContractState;
use cosmachain_smart_contracts::contracts::{
    cosmcare, cosmastar, cosmagigs, filmcore, caddiepro, cosmasocial,
    benefits_agreement, revenue_split, identity_bound_tx,
};

#[test]
fn test_cosmcare_contract() {
    let mut state = ContractState::new();
    let result = cosmcare(&mut state, vec!["user1".into(), "WellnessTier1".into()]);
    assert!(result.contains("CosmaCare updated"));
    assert_eq!(state.get("care_user1").unwrap(), "WellnessTier1");
}

#[test]
fn test_cosmastar_contract() {
    let mut state = ContractState::new();
    let result = cosmastar(&mut state, vec!["user1".into(), "Boost50".into()]);
    assert!(result.contains("CosmaStar visibility boosted"));
    assert_eq!(state.get("visibility_user1").unwrap(), "Boost50");
}

#[test]
fn test_cosmagigs_contract() {
    let mut state = ContractState::new();
    let result = cosmagigs(&mut state, vec!["worker1".into(), "gig123".into(), "200".into()]);
    assert!(result.contains("CosmaGigs payout"));
    assert_eq!(state.get("gigpay_worker1_gig123").unwrap(), "200");
}

#[test]
fn test_filmcore_contract() {
    let mut state = ContractState::new();
    let result = filmcore(&mut state, vec!["creator1".into(), "projectA".into(), "500".into()]);
    assert!(result.contains("FilmCore royalty"));
    assert_eq!(state.get("royalty_creator1_projectA").unwrap(), "500");
}

#[test]
fn test_caddiepro_contract() {
    let mut state = ContractState::new();
    let result = caddiepro(&mut state, vec!["caddie1".into(), "eventX".into(), "95".into()]);
    assert!(result.contains("CaddiePro score"));
    assert_eq!(state.get("score_caddie1_eventX").unwrap(), "95");
}

#[test]
fn test_cosmasocial_contract() {
    let mut state = ContractState::new();
    let result = cosmasocial(&mut state, vec!["user1".into(), "HighEngagement".into()]);
    assert!(result.contains("CosmaSocial engagement updated"));
    assert_eq!(state.get("engagement_user1").unwrap(), "HighEngagement");
}

#[test]
fn test_benefits_agreement_contract() {
    let mut state = ContractState::new();
    let result = benefits_agreement(&mut state, vec!["user1".into(), "PTO".into()]);
    assert!(result.contains("Benefit PTO applied"));
    assert_eq!(state.get("benefit_user1").unwrap(), "PTO");
}

#[test]
fn test_revenue_split_contract() {
    let mut state = ContractState::new();
    let result = revenue_split(&mut state, vec!["stylist1".into(), "client1".into(), "100".into()]);
    assert!(result.contains("Revenue split"));
    assert_eq!(state.get("stylist_pay_stylist1").unwrap(), "88");
    assert_eq!(state.get("platform_fee_client1").unwrap(), "12");
}

#[test]
fn test_identity_bound_tx_contract() {
    let mut state = ContractState::new();
    let result = identity_bound_tx(&mut state, vec!["user1".into(), "TXDATA".into()]);
    assert!(result.contains("Identity-bound transaction"));
    assert_eq!(state.get("idtx_user1").unwrap(), "TXDATA");
}


