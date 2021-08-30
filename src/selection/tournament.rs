use rand::{thread_rng, seq::SliceRandom};
use std::cmp::Ordering;

pub fn tournament_selection(fitness_values: &Vec<f32>, num_parents: usize, tournament_size: usize) -> Vec<usize> {
  let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
  for (i, &value) in fitness_values.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };

  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
      let mut rng = thread_rng();
      let mut current_tournament = fitness_values_with_index.choose_multiple(&mut rng, tournament_size).cloned().collect::<Vec<(f32, usize)>>();
      current_tournament.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
      selected_indices.push(current_tournament[tournament_size-1].1);
  };

  selected_indices
}
