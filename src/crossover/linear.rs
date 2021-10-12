pub fn linear_crossover(parent1: f32, parent2: f32, parameters: &Vec<(f32, f32)>) -> Vec<f32> {
    let mut offsprings = Vec::with_capacity(parameters.len());
    for &(alpha, beta) in parameters.iter() {
        offsprings.push(alpha * parent1 + beta * parent2);
    }
    offsprings
}
