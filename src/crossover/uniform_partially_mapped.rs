use super::{check_continuous, check_length};

use rand::{
    distributions::Uniform,
    prelude::{Distribution, IteratorRandom},
    rngs::StdRng,
    SeedableRng,
};

pub fn uniform_partially_mapped_crossover(
    parent1: &Vec<usize>,
    parent2: &Vec<usize>,
    probability: f64,
    seed: Option<u64>,
) -> (Vec<usize>, Vec<usize>) {
    check_length(parent1, parent2);

    let n = parent1.len();
    if !check_continuous(parent1) || !check_continuous(parent2) {
        panic!("Vectors must contain continuous unique values");
    }

    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    let mut selected = (0..n).map(|x| x).choose_multiple(&mut prng, 2);
    selected.sort();
    let (mut idx1, mut idx2) = (parent1.clone(), parent2.clone());
    for i in 0..n {
        idx1[parent1[i]] = i;
        idx2[parent2[i]] = i;
    }
    let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
    let mask = Uniform::from(0.0..1.0);
    for i in selected[0]..=selected[1] {
        if mask.sample(&mut prng) > probability {
            continue;
        }

        let (val1, val2) = (child1[i], child2[i]);
        child1[i] = val2;
        child1[idx1[val2]] = val1;
        child2[i] = val1;
        child2[idx2[val1]] = val2;

        let temp = (idx1[val1], idx1[val2]);
        idx1[val1] = temp.1;
        idx1[val2] = temp.0;
        let temp = (idx2[val1], idx2[val2]);
        idx2[val1] = temp.1;
        idx2[val2] = temp.0;
    }

    (child1, child2)
}
