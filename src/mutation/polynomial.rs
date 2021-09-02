use rand::{Rng, rngs::StdRng, SeedableRng};

pub fn polynomial_mutation(individual: f32, distribution_index: f32, seed: Option<u64>) -> f32 {
  let random_value = (match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  }).gen_range(0.0..1.0);

  let caluclate_perturbation_factor = |random_value: f32| -> f32 {
    if random_value < 0.5 {
      return (2.0*random_value).powf(1.0/(distribution_index + 1.0)) - 1.0;
    }
    return 1.0 - (2.0*(1.0 - random_value).powf(1.0/(distribution_index + 1.0)));
  };

  individual + (2.0*random_value - 1.0)*caluclate_perturbation_factor(random_value)
}
