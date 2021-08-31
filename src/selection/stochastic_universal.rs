use rand::{thread_rng, Rng};

pub fn stochastic_universal_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
  let sum_of_fitness = fitness_values.iter().sum::<f32>();
  let mut fitness_scale:Vec<f32> = Vec::new();
  for (idx, &val) in fitness_values.iter().enumerate() {
    if idx == 0 {
      fitness_scale.push(val);
    }
    else {
      fitness_scale.push(val + fitness_scale[idx-1]);
    }
  };

  let fitness_step = sum_of_fitness / num_parents as f32;
  let mut random_offset = thread_rng().gen_range(0.0..fitness_step);
  let mut current_offset = 0usize;
  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
    while fitness_scale[current_offset] < random_offset {
      current_offset += 1;
    }
    selected_indices.push(current_offset);
    current_offset += 1;
    random_offset += fitness_step;
  };
  selected_indices
}
