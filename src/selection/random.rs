use rand::{rngs::StdRng, SeedableRng, Rng};

pub fn random_selection(fitness_values: &Vec<f32>, num_parents: usize, seed: Option<u64>) -> Vec<usize> {
  let population_size = fitness_values.len();
  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
      let val = prng.gen_range(0..population_size);
      selected_indices.push(val);
  };
  selected_indices
}
