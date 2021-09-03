use std::cmp::Ordering;
use rand::{rngs::StdRng, SeedableRng, Rng};

/**
## Description
Rank Selection is a form of proportionate selection. It generates a roulette wheel where area occupied by a particular individual is proportional to its rank in fitness vector when sorted in non decreasing order.

For each of the `num_parents` iterations a random number between `0.0..1.0` is generated which then decides the location at which roulette wheel will stop and that particular individual's index is added to vector of selected individuals.

### Note

- Individuals with same fitness value are given the same ranks.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return

The return value is a `Vec<usize>` pointing to the selected indices.

## Example
```rust
  use genx::selection::rank_selection;
  let num_parents:usize = 10;
  let fitness_values = vec![10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

  let result = rank_selection(&fitness_values, num_parents, None);
```
*/
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
