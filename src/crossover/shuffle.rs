use rand::{SeedableRng, prelude::SliceRandom, rngs::StdRng};

use super::{check_length, single_point_crossover};

/**
## Description:
Shuffle Crossover selects the two parents for crossover. It firstly randomly shuffles the genes in
the both parents but in the same way. Then it applies the 1-Point crossover technique by randomly
selecting a point as crossover point and then combines both parents to create two offspring. After
performing 1-point crossover the genes in offspring are then unshuffled in same way as they have
been shuffled.

### Note:
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `Vec<bool>`

## Example:
```rust
use genx::crossover::shuffle_crossover;

let parent1 = vec![true, false, false, true, true, false, false, true];
let parent2 = vec![true, true, true, false, true, false, true, true];
let (child1, child2) = shuffle_crossover(&parent1, &parent2, None);
```
 */
pub fn shuffle_crossover(parent1: &Vec<bool>, parent2: &Vec<bool>, seed: Option<u64>) -> (Vec<bool>, Vec<bool>) {
  check_length(parent1, parent2);

  let mut prng = match seed {
    Some(seed) => StdRng::seed_from_u64(seed),
    None => StdRng::from_entropy(),
  };


  let mut indices = (0..parent1.len()).collect::<Vec<usize>>();
  indices.shuffle(&mut prng);
  let shuffled_parent1 = indices.iter().map(|&i| parent1[i]).collect::<Vec<bool>>();
  let shuffled_parent2 = indices.iter().map(|&i| parent2[i]).collect::<Vec<bool>>();

  let (shuffled_child1, shuffled_child2) = single_point_crossover(&shuffled_parent1, &shuffled_parent2, seed);

  let mut child1 = (0..parent1.len()).map(|_| false).collect::<Vec<bool>>();
  let mut child2 = (0..parent2.len()).map(|_| false).collect::<Vec<bool>>();

  for (idx, &index) in indices.iter().enumerate() {
    child1[index] = shuffled_child1[idx];
    child2[index] = shuffled_child2[idx];
  };

  return (child1, child2);
}
