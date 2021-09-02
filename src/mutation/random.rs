use rand::{Rng, rngs::StdRng, SeedableRng};

pub fn random_mutation(individual: f32, perturbation_factor: f32, seed: Option<u64>) -> f32 {
  let random_value = (match seed {
    Some(val) => StdRng::seed_from_u64(val),
    None => StdRng::from_entropy()
  }).gen_range(0.0..1.0);

  individual + (2.0*random_value - 1.0)*perturbation_factor
}
