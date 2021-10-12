use rand::{Rng, SeedableRng, prelude::StdRng};

pub fn simulated_binary_crossover(parent1: f32, parent2: f32, operator: f32, seed: Option<u64>) -> (f32, f32) {
  let mut prng = match seed {
      Some(val) => StdRng::seed_from_u64(val),
      None => StdRng::from_entropy(),
  };

  let beta = |ui: f32| {
    if ui <= 0.5 {
      return  (2.0*ui).powf(1.0/(operator + 1.0));
    }
    else {
      return (1.0/(2.0*(1.0 - ui))).powf(1.0/(operator + 1.0));
    }
  };

  let ui = prng.gen::<f32>();
  let beta_ui = beta(ui);
  (0.5*((parent1+parent2) - beta_ui*(parent1-parent2)), 0.5*((parent1+parent2) + beta_ui*(parent1-parent2)))
}
