use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

/**
## Description
Swap mutation is a mutation only for vector encoded individuals.
Given the `individual` it randomly generates two indices and then swaps the value at those two indices.

_Note: The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results._

## Example
```rust
  use genx::mutation::swap_mutation;
  let mut individual = vec![false, true, false, false,
                            true, true, true, false, false, true, false,
                            false, true, false, false, true];
  let original_individual = individual.clone();
  swap_mutation(&mut individual, Some(11));
  assert_ne!(original_individual, individual);
```
*/
pub fn swap_mutation<T>(individual: &mut Vec<T>, seed: Option<u64>) -> Result<(), &'static str>
where
    T: Copy,
{
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    if individual.len() < 2 {
        return Err("Need atleast two bits for swapping");
    }

    let mut fitness_values_with_index: Vec<(T, usize)> = Vec::new();
    for (i, &value) in individual.iter().enumerate() {
        fitness_values_with_index.push((value, i));
    }

    let selected_indices = fitness_values_with_index
        .choose_multiple(&mut prng, 2)
        .map(|&x| x)
        .collect::<Vec<(T, usize)>>();
    individual[selected_indices[0].1] = selected_indices[1].0;
    individual[selected_indices[1].1] = selected_indices[0].0;

    Ok(())
}
