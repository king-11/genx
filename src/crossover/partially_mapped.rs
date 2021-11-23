use super::{check_continuous, check_length};

use rand::{SeedableRng, prelude::IteratorRandom, rngs::StdRng};

/**
## Description:
It transmits ordering and values information from the parent strings to the offspring. A portion of
one parent string is mapped onto a portion of the other parent string and the remaining information
is exchanged.

### Note:
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `Vec<usize>`

## Example:
```rust
use genx::crossover::partially_mapped_crossover;

let parent1 = vec![1, 3, 4, 7, 0, 2, 6, 5];
let parent2 = vec![2, 3, 4, 0, 7, 6, 1, 5];
let (child1, child2) = partially_mapped_crossover(&parent1, &parent2, None);
```
 */
pub fn partially_mapped_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, seed: Option<u64>) -> (Vec<usize>, Vec<usize>) {
  check_length(parent1, parent2);

  let n = parent1.len();
  if !check_continuous(parent1) || !check_continuous(parent2) {
    panic!("Vectors must contain continuous unique values");
  }

  let mut prng = match seed {
    Some(seed) => StdRng::seed_from_u64(seed),
    None => StdRng::from_entropy(),
  };

  let mut selected = (0..n).map(|x| x).choose_multiple(&mut prng, 2);
  selected.sort();
  let (mut idx1, mut idx2) = (parent1.clone(), parent2.clone());
  for i in 0..n {
    idx1[parent1[i]] = i;
    idx2[parent2[i]] = i;
  }
  let (mut child1, mut child2) = (parent1.clone(), parent2.clone());

  for i in selected[0]..=selected[1] {
    let (val1, val2) = (child1[i], child2[i]);

    child1[i] = val2;
    child1[idx1[val2]] = val1;
    child2[i] = val1;
    child2[idx2[val1]] = val2;

    let temp = (idx1[val1], idx1[val2]);
    idx1[val1] = temp.1;
    idx1[val2] = temp.0;
    let temp = (idx2[val1], idx2[val2]);
    idx2[val1] = temp.1;
    idx2[val2] = temp.0;
  }

  (child1, child2)
}
