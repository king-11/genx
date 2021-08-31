use rand::{seq::SliceRandom, rngs::StdRng, SeedableRng};
use std::cmp::Ordering;

pub fn tournament_selection(fitness_values: &Vec<f32>, num_parents: usize, tournament_size: usize, seed: Option<u64>) -> Vec<usize> {
  let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
  for (i, &value) in fitness_values.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };

  let mut selected_indices:Vec<usize> = Vec::new();
  for idx in 0..num_parents {
      let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val*(idx as u64)),
        None => StdRng::from_entropy()
      };
      let mut current_tournament = fitness_values_with_index.choose_multiple(&mut prng, tournament_size).cloned().collect::<Vec<(f32, usize)>>();
      current_tournament.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
      selected_indices.push(current_tournament[0].1);
  };

  selected_indices
}
