use std::cmp::{Ordering, min};

/**
## Description
Steady State Selection is a sorting based selection method. It will always select the top `num_parents` in terms of fitness value from the pool of individuals.

_Note: If `num_parents` is greater than length of `fitness_values` vector (n), then only indices from `0..n-1` will be returned._

## Return

The return value is a `Vec<usize>` pointing to the selected indices.

## Example
```rust
  use genx::selection::steady_state_selection;
  let num_parents:usize = 10;
  let fitness_values = vec![10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

  let result = steady_state_selection(&fitness_values, num_parents);
```
*/
pub fn steady_state_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
  let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
  let population_size = fitness_values.len();
  for (i, &value) in fitness_values.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };
  fitness_values_with_index.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
  let selected_indices = fitness_values_with_index.iter().map(|&a| a.1).collect::<Vec<usize>>();
  selected_indices[0..min(num_parents, population_size)].to_vec()
}
