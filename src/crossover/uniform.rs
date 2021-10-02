use itertools::multizip;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::mem::swap;

use super::check_length;

pub fn uniform_crossover(
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
    let mask = (0..n).map(|_| prng.gen_bool(0.5)).collect::<Vec<bool>>();
    let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
    for (val1, val2, mask_val) in multizip((&mut child1, &mut child2, mask)) {
        if mask_val {
            swap(val1, val2);
        }
    }

    return (child1, child2);
}
