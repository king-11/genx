use rand::{rngs::StdRng, Rng, SeedableRng};

use super::check_length;

pub fn single_point_crossover(
    parent1: &Vec<bool>,
    parent2: &Vec<bool>,
    seed: Option<u64>,
) -> (Vec<bool>, Vec<bool>) {
    check_length(parent1, parent2);

    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };


    let n = parent1.len();
    let (mut child1, mut child2) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let idx = prng.gen_range(0..parent1.len());
    for i in 0..parent1.len() {
        if i < idx {
            child1.push(parent1[i]);
            child2.push(parent2[i]);
        } else {
            child1.push(parent2[i]);
            child2.push(parent1[i]);
        }
    }

    return (child1, child2);
}
