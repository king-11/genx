use std::cmp::min;

use rand::{SeedableRng, prelude::IteratorRandom, rngs::StdRng};

/**
## Description
Random Selection is the simplest form of selection which randomly
generates an index from the provided `fitness_values` vector and that
is the selection. Same procedure is done until we have `num_parents` selected individuals.

_Note: The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results._

## Return

The return value is a `Vec<usize>` pointing to the selected indices.

## Example
```rust
  use genx::selection::random_selection;
  let num_parents:usize = 10;
  let fitness_values = vec![10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

  let result = random_selection(fitness_values.len(), num_parents, None);
```
*/
pub fn random_selection(population_size: usize, num_parents: usize, seed: Option<u64>) -> Vec<usize> {
  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  (0..population_size).map(|x| x).choose_multiple(&mut prng, min(num_parents, population_size))
}
