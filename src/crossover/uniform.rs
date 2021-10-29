use itertools::multizip;
use rand::{SeedableRng, distributions::Uniform, prelude::Distribution, rngs::StdRng};
use std::mem::swap;

use super::check_length;

pub fn uniform_crossover(
    parent1: &Vec<bool>,
    parent2: &Vec<bool>,
    probability: f64,
    seed: Option<u64>,
) -> (Vec<bool>, Vec<bool>) {
    check_length(parent1, parent2);

    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };


    let mask = Uniform::from(0.0..1.0);
    let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
    for (val1, val2) in multizip((&mut child1, &mut child2)) {
        if mask.sample(&mut prng) < probability {
            swap(val1, val2);
        }
    }

    return (child1, child2);
}
