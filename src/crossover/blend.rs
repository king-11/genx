use rand::{prelude::StdRng, Rng, SeedableRng};

/**
## Description:
It uses the following formula to generate offsprings from parents:
    C1 = (1-γ)P1 + γP2
    C2 = (1-γ)P2 + γP1
    Here, γ = (1+2ɑ)r - ɑ [ɑ is a fixed value, say 0.5 and r is a random value between 0 and 1]

### Note:
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `f32`

## Example:
```rust
use genx::crossover::blend_crossover;

let (parent1, parent2) = (13.37, 9.43);
let (child1, child2) = blend_crossover(parent1, parent2, 0.5, None);
```
 */
pub fn blend_crossover(parent1: f32, parent2: f32, alpha: f32, seed: Option<u64>) -> (f32, f32) {
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    let gamma: f32 = (1.0 + 2.0 * alpha) * prng.gen::<f32>() - alpha;
    (
        (1.0 - gamma) * parent1 + gamma * parent2,
        (1.0 - gamma) * parent2 + gamma * parent1,
    )
}
