use rand::{rngs::StdRng, SeedableRng, Rng};

pub fn flipping_mutation(individual: &mut Vec<bool>, mutation_probability: f32, seed: Option<u64>) -> Result<(), &'static str> {
  if mutation_probability < 0.0 || mutation_probability > 1.0 {
    return Err("mutation_probability should lie between 0.0 and 1.0 inclusive");
  }

  let mut prng = match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  };
  for val in individual.iter_mut() {
    let random_probability = prng.gen_range(0.0..1.0);
    if random_probability < mutation_probability {
      *val = !(*val);
    }
  };

  Ok(())
}
