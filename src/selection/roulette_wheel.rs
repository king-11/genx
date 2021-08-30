use rand::{thread_rng, Rng};

pub fn roulette_wheel_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
  let sum_of_fitness = fitness_values.iter().sum::<f32>();
  let normalized_probabilities = fitness_values.iter().map(|&a| a/sum_of_fitness).collect::<Vec<f32>>();

  let mut selected_indices:Vec<usize> = Vec::new();
  for _ in 0..num_parents {
      let val = thread_rng().gen();
      let mut cummulative_probability = 0f32;
      for (index, _) in fitness_values.iter().enumerate() {
          cummulative_probability += normalized_probabilities[index];
          if cummulative_probability >= val {
              selected_indices.push(index);
              break;
          }
      };
  };
  selected_indices
}
