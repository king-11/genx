use itertools::multizip;
use rand::{SeedableRng, distributions::Uniform, prelude::Distribution, rngs::StdRng};
use std::mem::swap;

use super::check_length;

/**
## Description:
Uniform crossover selects the two parents for crossover. It creates two child offspring of n genes
selected from both of the parents uniformly. The random real number decides whether the first child
select the ith genes from first or second parent.

### Note:
- The function takes a float value `probability` in the range [0.0 - 1.0] representing the bias.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `Vec<bool>`

## Example:
```rust
use genx::crossover::uniform_crossover;

let parent1 = vec![true, false, false, true, true, false, false, true];
let parent2 = vec![true, true, true, false, true, false, true, true];
let (child1, child2) = uniform_crossover(&parent1, &parent2, 0.6, None);
```
 */
pub fn uniform_crossover(
    parent1: &Vec<bool>,
    parent2: &Vec<bool>,
    probability: f64,
    seed: Option<u64>,
) -> (Vec<bool>, Vec<bool>) {
    check_length(parent1, parent2);

    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };


    let mask = Uniform::from(0.0..1.0);
    let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
    for (val1, val2) in multizip((&mut child1, &mut child2)) {
        if mask.sample(&mut prng) < probability {
            swap(val1, val2);
        }
    }

    return (child1, child2);
}
