use rand::{Rng, SeedableRng, prelude::StdRng};

/**
## Description:
Simulated binary crossover uses probability density function that simulates the single-point
crossover in binary-coded GAs.

### Note:
- The function takes a `f32` parameter `operator` denoting SBX crossover distribution factor.
- The function can also take in an optional `seed` value of type `Option<u64>` for deterministic results.

## Return:
The return value is a tuple containing two offsprings of type `f32`

## Example:
```rust
use genx::crossover::simulated_binary_crossover;

let (parent1, parent2) = (11.19, 20.97);
let (child1, child2) = simulated_binary_crossover(parent1, parent2, 0.5, None);
```
 */
pub fn simulated_binary_crossover(parent1: f32, parent2: f32, operator: f32, seed: Option<u64>) -> (f32, f32) {
  let mut prng = match seed {
      Some(val) => StdRng::seed_from_u64(val),
      None => StdRng::from_entropy(),
  };

  let beta = |ui: f32| {
    if ui <= 0.5 {
      return  (2.0*ui).powf(1.0/(operator + 1.0));
    }
    else {
      return (1.0/(2.0*(1.0 - ui))).powf(1.0/(operator + 1.0));
    }
  };

  let ui = prng.gen::<f32>();
  let beta_ui = beta(ui);
  (0.5*((parent1+parent2) - beta_ui*(parent1-parent2)), 0.5*((parent1+parent2) + beta_ui*(parent1-parent2)))
}
