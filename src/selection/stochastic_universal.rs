use rand::{rngs::StdRng, SeedableRng, Rng};

/**
## Description
Stochastic Universal Selection/Sampling is also a proportionate selection method very much like roulette wheel selection. The major differences is that it selects the required number of individuals (`num_parents`) in a single spin of the wheel which allows for population diversity.

To know more about stochastic universal sampling refer the [wikipedia page](https://en.wikipedia.org/wiki/Stochastic_universal_sampling)

### Note

- Individuals with same fitness value occupy same area on roulette wheel.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return

The return value is a `Vec<usize>` pointing to the selected indices.

## Example
```rust
  use genx::selection::stochastic_universal_selection;
  let num_parents:usize = 10;
  let fitness_values = vec![10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];
  let result = stochastic_universal_selection(&fitness_values, num_parents, None);
```
*/
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
