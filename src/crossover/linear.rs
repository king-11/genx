
/**
## Description:
It uses the linear combination of the present chromosomes to produce new children.
SSuppose P<sub>1</sub> and P<sub>2</sub> are the parent values. Corresponding offspring chromosome values can be
obtained as C<sub>i</sub> = ɑ<sub>i</sub>P<sub>1</sub> + β<sub>i</sub>P<sub>2</sub>, where i = 1,2,3...n

### Note:
- The function takes a vector of (ɑ, β) pairs as a parameter.

## Return:
The return value is a vector `Vec<f32>` containing children for each parameter.

## Example:
```rust
use genx::crossover::linear_crossover;

let (parent1, parent2) = (11.13, 12.19);
let params = vec![(0.5, 0.5), (0.6, -0.4)];
let children = linear_crossover(parent1, parent2, &params);
```
 */
pub fn linear_crossover(parent1: f32, parent2: f32, parameters: &Vec<(f32, f32)>) -> Vec<f32> {
    let mut offsprings = Vec::with_capacity(parameters.len());
    for &(alpha, beta) in parameters.iter() {
        offsprings.push(alpha * parent1 + beta * parent2);
    }
    offsprings
}
