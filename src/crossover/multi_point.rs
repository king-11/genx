use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use super::check_length;

/**
## Description:
It uses the random crossover points to combine the parents same as per 1-Point crossover.
To provide the great combination of parents it selects more than one crossover points to create the
offspring or child.

### Note:
- The function takes an integer `k` denoting the number of crossover points.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `Vec<bool>`

## Example:
```rust
use genx::crossover::multi_point_crossover;

let parent1 = vec![true, false, false, true, true, false, false, true];
let parent2 = vec![true, true, true, false, true, false, true, true];
let (child1, child2) = multi_point_crossover(&parent1, &parent2, 3, None);
```
 */
pub fn multi_point_crossover(
    parent1: &Vec<bool>,
    parent2: &Vec<bool>,
    k: usize,
    seed: Option<u64>,
) -> (Vec<bool>, Vec<bool>) {
    check_length(parent1, parent2);

    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };


    let n = parent1.len();
    let mut indices = (0..n)
        .collect::<Vec<usize>>()
        .choose_multiple(&mut prng, k)
        .map(|&x| x)
        .collect::<Vec<usize>>();

    indices.sort();
    let (mut child1, mut child2) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let mut child_idx: usize = 0;
    for (i, &idx) in indices.iter().enumerate() {
        if i & 1 == 0 {
            while child_idx < idx {
                child1.push(parent1[child_idx]);
                child2.push(parent2[child_idx]);
                child_idx += 1;
            }
        } else {
            while child_idx < idx {
                child1.push(parent2[child_idx]);
                child2.push(parent1[child_idx]);
                child_idx += 1;
            }
        }
    }

    if child_idx < n {
        while child_idx < n {
            if k & 1 == 0 {
                child1.push(parent1[child_idx]);
                child2.push(parent2[child_idx]);
            } else {
                child1.push(parent2[child_idx]);
                child2.push(parent1[child_idx]);
            }
            child_idx += 1;
        }
    }

    return (child1, child2);
}
