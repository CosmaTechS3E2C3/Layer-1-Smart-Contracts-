use crate::core::smart_contracts::engine::contract_state::ContractState;

pub fn revenue_split(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 3 {
        return "Invalid args".to_string();
    }

    let stylist = &args[0];
    let client = &args[1];
    let amount = args[2].parse::<u64>().unwrap_or(0);

    let stylist_amount = (amount as f64 * 0.88) as u64;
    let platform_amount = amount - stylist_amount;

    state.set(&format!("stylist_pay_{}", stylist), &stylist_amount.to_string());
    state.set(&format!("platform_fee_{}", client), &platform_amount.to_string());

    format!(
        "Revenue split: stylist {} gets {}, platform gets {}",
        stylist, stylist_amount, platform_amount
    )
}
