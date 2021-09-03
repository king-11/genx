use rand::{rngs::StdRng, SeedableRng, Rng};

/**
## Description
Roulette Wheel Selection is a form of proportionate selection. It generates a roulette wheel where area occupied by a particular individual is proportional to its fitness values.

For each of the `num_parents` iterations a random number between `0.0..1.0` is generated which then decides the location at which roulette wheel will stop and that particular individual's index is added to vector of selected individuals.

### Note

- Individuals with same fitness value occupy same area on roulette wheel.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return

The return value is a `Vec<usize>` pointing to the selected indices.

## Example
```rust
  use genx::selection::roulette_wheel_selection;
  let num_parents:usize = 10;
  let fitness_values = vec![10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

  let result = roulette_wheel_selection(&fitness_values, num_parents, None);
```
*/
pub fn roulette_wheel_selection(fitness_values: &Vec<f32>, num_parents: usize, seed: Option<u64>) -> Vec<usize> {
  let sum_of_fitness = fitness_values.iter().sum::<f32>();
  let normalized_probabilities = fitness_values.iter().map(|&a| a/sum_of_fitness).collect::<Vec<f32>>();

  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
      let val = prng.gen();
      let mut cummulative_probability = 0f32;
      for (index, _) in fitness_values.iter().enumerate() {
          cummulative_probability += normalized_probabilities[index];
          if cummulative_probability >= val {
              selected_indices.push(index);
              break;
          }
      };
  };
  selected_indices
}
