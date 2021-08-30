use rand::{thread_rng, Rng};

pub fn random_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
  let population_size = fitness_values.len();
  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
      let val = thread_rng().gen_range(0..population_size);
      selected_indices.push(val);
  };
  selected_indices
}
