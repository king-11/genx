use rand::{rngs::StdRng, SeedableRng, seq::SliceRandom};

pub fn swap_mutation(individual: &mut Vec<bool>, seed: Option<u64>) -> Result<(), &'static str> {
  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };

  if individual.len() < 2 {
    return Err("Need atleast two bits for swapping");
  }

  let mut fitness_values_with_index : Vec<(bool, usize)> = Vec::new();
  for (i, &value) in individual.iter().enumerate() {
      fitness_values_with_index.push((value, i));
  };

  let selected_indices = fitness_values_with_index.choose_multiple(&mut prng, 2).cloned().collect::<Vec<(bool,usize)>>();
  individual[selected_indices[0].1] = selected_indices[1].0;
  individual[selected_indices[1].1] = selected_indices[0].0;

  Ok(())
}
