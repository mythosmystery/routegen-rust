use super::array_manipulation::{get_random_int, swap};

/// Apply Mutation
pub fn apply_mutation(route: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    const TWO_OPT_PROBABILITY: f64 = 0.5;
    if rand::random::<f64>() < TWO_OPT_PROBABILITY {
        two_opt(route)
    } else {
        mutate(route)
    }
}

/// 2-opt Mutation
pub fn two_opt(route: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut index1 = get_random_int(route.len());
    let mut index2 = get_random_int(route.len());

    while index2 == index1 {
        index2 = get_random_int(route.len());
    }

    if index1 > index2 {
        std::mem::swap(&mut index1, &mut index2);
    }

    let mut new_route = route[..index1].to_vec();
    new_route.extend(route[index1..=index2].iter().rev());
    new_route.extend(&route[index2 + 1..]);

    new_route
}

/// Mutate
pub fn mutate(route: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    swap(
        route,
        get_random_int(route.len()),
        get_random_int(route.len()),
    );
    route.clone()
}

/// PMX Crossover (Partially Mapped Crossover)
pub fn pmx_crossover(parent1: &Vec<(f64, f64)>, parent2: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut child = vec![(0.0, 0.0); parent1.len()];
    let start = get_random_int(parent1.len());
    let end = (get_random_int(parent1.len()) + start) % parent1.len();

    let mut mapping = std::collections::HashMap::new();

    for i in start..end {
        child[i] = parent1[i];
        mapping.insert(format!("{:?}", parent1[i]), parent2[i]);
    }

    for i in 0..parent1.len() {
        if child[i] == (0.0, 0.0) {
            let mut insert = parent2[i];

            while mapping.contains_key(format!("{:?}", &insert)) {
                insert = mapping[&insert];
            }

            child[i] = insert;
        }
    }

    child
}
