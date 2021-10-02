use rand::{seq::SliceRandom, rngs::StdRng, SeedableRng};
use std::cmp::{Ordering, min};

/**
## Description
Tournament Selection is a randomized sorting based selection method. We conduct `num_parents` tournaments where we randomly select `tournament_size` individuals. Out of those selected for a particular tournament, the fittest one is added to the vector of selected indices.

###  Note:

- If `tournament_size` is greater than length of `fitness_values` vector (n), then overall fittest individual gets selected from all the tournaments.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return

The return value is a `Vec<usize>` pointing to the selected indices.

## Example
```rust
  use genx::selection::tournament_selection;
  let num_parents:usize = 10;
  let fitness_values = vec![10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

  let result = tournament_selection(&fitness_values, num_parents, 4, None);
```
*/
pub fn tournament_selection(fitness_values: &Vec<f32>, num_parents: usize, tournament_size: usize, seed: Option<u64>) -> Vec<usize> {
  let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
  for (i, &value) in fitness_values.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };

  let number_individuals = fitness_values.len();
  let mut selected_indices:Vec<usize> = Vec::new();
  for idx in 0..num_parents {
      let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val*(idx as u64)),
        None => StdRng::from_entropy()
      };
      let mut current_tournament = fitness_values_with_index.choose_multiple(&mut prng, min(tournament_size, number_individuals)).map(|&x| x).collect::<Vec<(f32, usize)>>();
      current_tournament.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
      selected_indices.push(current_tournament[0].1);
  };

  selected_indices
}
