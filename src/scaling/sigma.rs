pub fn sigma_scaling(fitness_values: &mut Vec<f32>, scaling_factor: f32) {
  let average_fitness = fitness_values.iter().sum::<f32>() / (fitness_values.len() as f32);
  let standard_deviation = fitness_values
    .iter()
    .map(|x| (x - average_fitness).powi(2))
    .sum::<f32>();
  let standard_deviation = (standard_deviation / (fitness_values.len() as f32)).sqrt();
  let worst_fitness = average_fitness - standard_deviation * scaling_factor;
  for x in fitness_values {
    if *x <= worst_fitness {
      *x = 0.0;
    }
    else {
      *x = *x - worst_fitness;
    }

    *x = *x + 1.0;
  }
}
