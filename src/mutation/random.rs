use rand::{Rng, rngs::StdRng, SeedableRng};

/**
## Description
Random mutation is a mutation only for real encoded individuals.
Given the `individual` and `perturbation_factor` it generates a random value
in range `0.0..1.0` which is then used to linearly mutate the individual.

#### Note
- A large value for `perturbation_factor` will make stark mutation for all values.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return
Finally returned value is `individual + (2.0*random_value - 1.0)*perturbation_factor`

## Example
```rust
  use genx::mutation::random_mutation;

  let individual = 29.11;
  let result = random_mutation(individual, 4.2, Some(42));
  assert_ne!(individual, result);
```
*/
pub fn random_mutation(individual: f32, perturbation_factor: f32, seed: Option<u64>) -> f32 {
  let random_value = (match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  }).gen_range(0.0..1.0);

  individual + (2.0*random_value - 1.0)*perturbation_factor
}
