fn main() {
    // CNF風の二次元リスト
    let input = vec![vec![1, 2], vec![1, -2], vec![-1, 2], vec![-1, -2]];

    let result = DPLL(input);
    
    println!("{:?}", result);
}

type CL = Vec<Vec<i8>>;

fn DPLL(clause_list: CL) -> bool {
    let result = apply_unit_rule(clause_list);
    if result.is_empty() {
        return true;
    };
    if result.contains(&vec![]) {
        return false;
    };
    let v = result[0][0];
    DPLL(assign(result.clone(), v)) || DPLL(assign(result, -v))
}

fn apply_unit_rule(clause_list: CL) -> CL {
    match clause_list.clone().iter().find(|list| list.len() == 1) {
        Some(v) => apply_unit_rule(assign(clause_list, v[0])),
        None => clause_list,
    }
}

fn assign(clause_list: CL, variable: i8) -> CL {
    clause_list
        .into_iter()
        .filter(|list| list.iter().all(|&v| v != variable))
        .map(|list| {
            let list = list.into_iter().filter(|&v| -v != variable).collect();
            return list;
        })
        .collect()
}
