use std::cmp::Ordering;
use rand::{rngs::StdRng, SeedableRng, Rng};

pub fn rank_selection(fitness_values: &Vec<f32>, num_parents: usize, seed: Option<u64>) -> Vec<usize> {
  let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
  for (i, &value) in fitness_values.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };
  fitness_values_with_index.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
  let mut ranks:Vec<i32> = vec![1];
  let mut sum_of_fitness = 1;
  for (i, _) in fitness_values_with_index.iter().enumerate() {
      if i == 0 {
          continue;
      }
      if fitness_values_with_index[i].0 != fitness_values_with_index[i-1].0 {
          ranks.push(ranks[i-1] + 1);
      }
      else {
          ranks.push(ranks[i-1]);
      }
      sum_of_fitness += ranks[i];
  };
  let normalized_probabilities = ranks.iter().map(|&a| (a as f32)/(sum_of_fitness as f32)).collect::<Vec<f32>>();

  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
      let val = prng.gen();
      let mut cummulative_probability = 0f32;
      for (i, &(_, idx)) in fitness_values_with_index.iter().enumerate() {
          cummulative_probability += normalized_probabilities[i];
          if cummulative_probability >= val {
              selected_indices.push(idx);
              break;
          }
      };
  };
  selected_indices
}
