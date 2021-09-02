use std::{mem::swap};

use rand::{Rng, SeedableRng, rngs::StdRng};

pub fn inversion_mutation(individual: &mut Vec<bool>, seed: Option<u64>) {
  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };

  let length_of_individual = individual.len();

  let mut idx1 = prng.gen_range(0..length_of_individual);
  let mut idx2 = prng.gen_range(0..length_of_individual);

  if idx2 < idx1 {
    swap(&mut idx1, &mut idx2);
  }

  for idx in idx1..=idx2 {
    individual[idx] = !individual[idx];
  };
}
