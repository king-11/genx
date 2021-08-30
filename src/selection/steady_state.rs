use std::cmp::{Ordering, max};

pub fn steady_state_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
  let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
  let population_size = fitness_values.len();
  for (i, &value) in fitness_values.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };
  fitness_values_with_index.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
  let selected_indices = fitness_values_with_index.iter().map(|&a| a.1).collect::<Vec<usize>>();
  selected_indices[0..max(num_parents, population_size)].to_vec()
}
