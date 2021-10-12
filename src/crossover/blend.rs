use rand::{prelude::StdRng, Rng, SeedableRng};

pub fn blend_crossover(parent1: f32, parent2: f32, alpha: f32, seed: Option<u64>) -> (f32, f32) {
    let mut prng = match seed {
        Some(val) => StdRng::seed_from_u64(val),
        None => StdRng::from_entropy(),
    };

    let gamma: f32 = (1.0 + 2.0 * alpha) * prng.gen::<f32>() - alpha;
    (
        (1.0 - gamma) * parent1 + gamma * parent2,
        (1.0 - gamma) * parent2 + gamma * parent1,
    )
}
