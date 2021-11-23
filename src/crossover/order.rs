use super::{check_continuous, check_length};
use rand::{prelude::IteratorRandom, rngs::StdRng, SeedableRng};

/**
## Description:
It constructs an offspring by choosing a substring of one parent and preserving the relative order
of the elements of the other parent.

### Note:
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `Vec<usize>`

## Example:
```rust
use genx::crossover::order_crossover;

let parent1 = vec![1, 3, 4, 7, 0, 2, 6, 5];
let parent2 = vec![2, 3, 4, 0, 7, 6, 1, 5];
let (child1, child2) = order_crossover(&parent1, &parent2, None);
```
 */
pub fn order_crossover(
    parent1: &Vec<usize>,
    parent2: &Vec<usize>,
    seed: Option<u64>,
) -> (Vec<usize>, Vec<usize>) {
    check_length(parent1, parent2);
    if !check_continuous(parent1) || !check_continuous(parent2) {
        panic!("Vectors must contain continuous unique values");
    }

    let n = parent1.len();
    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    let mut selected = (0..n).map(|x| x).choose_multiple(&mut prng, 2);
    selected.sort();

    let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
    let (mut holes1, mut holes2) = (
        (0..n).map(|_| false).collect::<Vec<bool>>(),
        (0..n).map(|_| false).collect::<Vec<bool>>(),
    );

    for i in selected[0]..=selected[1] {
        holes1[parent2[i]] = true;
        holes2[parent1[i]] = true;

        child1[i] = parent2[i];
        child2[i] = parent1[i];
    }

    let (mut pos1, mut pos2) = (selected[1] + 1, selected[1] + 1);
    for i in 0..n {
        if !holes1.get(parent1[(i + selected[1] + 1) % n]).unwrap() {
            child1[pos1 % n] = parent1[(i + selected[1] + 1) % n];
            pos1 += 1;
        }
        if !holes2.get(parent2[(i + selected[1] + 1) % n]).unwrap() {
            child2[pos2 % n] = parent2[(i + selected[1] + 1) % n];
            pos2 += 1;
        }
    }

    (child1, child2)
}
