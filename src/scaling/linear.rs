use std::cmp::Ordering;

pub fn linear_scaling(fitness_values: &mut Vec<f32>, scaling_factor: f32) {
    let minimum_fitness = fitness_values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();
    let maximum_fitness = fitness_values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();
    let average_fitness = fitness_values.iter().sum::<f32>() / (fitness_values.len() as f32);
    if average_fitness == 0.0 {
        for x in fitness_values {
            *x = 1.0;
        }
        return;
    }
    let mut a = (average_fitness * (scaling_factor - 1.0)) / (maximum_fitness - average_fitness);
    let mut b = (average_fitness * (maximum_fitness - scaling_factor * average_fitness))
        / (maximum_fitness - average_fitness);

    if *minimum_fitness <= -1.0 * b / a {
        a = average_fitness / (average_fitness - minimum_fitness);
        b = -1.0 * minimum_fitness * average_fitness / (average_fitness - minimum_fitness);
    }

    let linear_function = |x:f32| a*x + b;
    for x in fitness_values {
      *x = linear_function(*x);
    }
}
