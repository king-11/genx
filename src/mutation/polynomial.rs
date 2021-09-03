use rand::{Rng, rngs::StdRng, SeedableRng};

/**
## Description
Polynomial mutation is a mutation only for real encoded individuals.
Given the `individual`, `distribution_index` and `max_perturbation` it generates a closure
which is an exponential polynomial function using the `distribution_index`. Given a randomly generated value
in range `0.0..1.0` to the closure it returns the `perturbation_factor`.

### Note
- A large value for `distribution_index` will make stark mutation for only a few values whereas a small `distribution_index` will cause uniform mutation for all randomly generated values (undesirable).
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return
Finally returned value is `individual + calculate_perturbation_factor(random_value)*max_perturbation`

## Example
```rust
  use genx::mutation::polynomial_mutation;

  let individual = 29.11;
  let result = polynomial_mutation(individual, 4.2, 4.0, Some(42));
  assert_ne!(individual, result);
```
*/
pub fn polynomial_mutation(individual: f32, distribution_index: f32, max_perturbation: f32, seed: Option<u64>) -> f32 {
  let random_value = (match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  }).gen_range(0.0..1.0);

  let calculate_perturbation_factor = |random_value: f32| -> f32 {
    if random_value < 0.5 {
      return (2.0*random_value).powf(1.0/(distribution_index + 1.0)) - 1.0;
    }
    return 1.0 - (2.0*(1.0 - random_value).powf(1.0/(distribution_index + 1.0)));
  };

  individual + calculate_perturbation_factor(random_value)*max_perturbation
}
