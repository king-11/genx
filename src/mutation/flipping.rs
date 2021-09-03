use rand::{rngs::StdRng, SeedableRng, Rng};

/**
## Description
Flipping mutation is a mutation only for binary encoded individuals.
Given the `mutation_probability` and `individual` it iterates through the boolean vector of `individual`
and generates a random number between `0.0` and `1.0`. If the number is less than `mutation_probability`
then we _flip_ the value at that index else it remains the same.

_Note: The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results._

## Return
The return value is a `Result<(), &'static str>` which will return error only if the `mutation_probability` is greater than `1`
## Example
```rust
  use genx::mutation::flipping_mutation;
  let mut individual = vec![false, true, false, false,
                            true, true, true, false, false, true, false,
                            false, true, false, false, true];
   let original_individual = individual.clone();
   match flipping_mutation(&mut individual, 0.5, Some(42)) {
     Ok(_) => (),
     Err(error) => panic!("{:?}", error)
   };
   assert_ne!(original_individual, individual);
```
*/
pub fn flipping_mutation(individual: &mut Vec<bool>, mutation_probability: f32, seed: Option<u64>) -> Result<(), &'static str> {
  if mutation_probability < 0.0 || mutation_probability > 1.0 {
    return Err("mutation_probability should lie between 0.0 and 1.0 inclusive");
  }

  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  for val in individual.iter_mut() {
    let random_probability = prng.gen_range(0.0..1.0);
    if random_probability < mutation_probability {
      *val = !(*val);
    }
  };

  Ok(())
}
