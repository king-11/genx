use rand::{rngs::StdRng, SeedableRng, Rng};

pub fn stochastic_universal_selection(fitness_values: &Vec<f32>, num_parents: usize, seed: Option<u64>) -> Vec<usize> {
  let sum_of_fitness = fitness_values.iter().sum::<f32>();
  let mut fitness_scale:Vec<f32> = Vec::new();
  for (idx, &val) in fitness_values.iter().enumerate() {
    if idx == 0 {
      fitness_scale.push(val);
    }
    else {
      fitness_scale.push(val + fitness_scale[idx-1]);
    }
  };

  let fitness_step = sum_of_fitness / num_parents as f32;

  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  let mut random_offset = prng.gen_range(0.0..fitness_step);
  let mut current_offset = 0usize;
  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
    while fitness_scale[current_offset] < random_offset {
      current_offset += 1;
    }
    selected_indices.push(current_offset);
    random_offset += fitness_step;
  };
  selected_indices
}
