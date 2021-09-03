use std::{mem::swap};

use rand::{Rng, SeedableRng, rngs::StdRng};

/**
## Description
Inversion mutation is a mutation only for binary encoded individuals.
Given the `individual` it randomly generates two indices and then inverts
the value between those indices of the individual.

_Note: The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results._

## Example
```rust
  use genx::mutation::inversion_mutation;
  let mut individual = vec![false, true, false, false,
                            true, true, true, false, false, true, false,
                            false, true, false, false, true];
  let original_individual = individual.clone();
  inversion_mutation(&mut individual, Some(42));
  assert_ne!(original_individual, individual);
```
*/
pub fn inversion_mutation(individual: &mut Vec<bool>, seed: Option<u64>) {
  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };

  let length_of_individual = individual.len();

  let mut idx1 = prng.gen_range(0..length_of_individual);
  let mut idx2 = prng.gen_range(0..length_of_individual);

  if idx2 < idx1 {
    swap(&mut idx1, &mut idx2);
  }

  for idx in idx1..=idx2 {
    individual[idx] = !individual[idx];
  };
}
