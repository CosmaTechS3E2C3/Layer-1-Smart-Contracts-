use crate::core::smart_contracts::engine::contract_state::ContractState;

pub fn filmcore(state: &mut ContractState, args: Vec<String>) -> String {
    if args.len() < 3 {
        return "Invalid args".to_string();
    }

    let creator = &args[0];
    let project = &args[1];
    let amount = &args[2];

    state.set(&format!("royalty_{}_{}", creator, project), amount);

    format!("FilmCore royalty {} assigned to {} for {}", amount, creator, project)
}
