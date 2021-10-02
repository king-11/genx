use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use super::check_length;

pub fn multi_point_crossover(
    parent1: &Vec<bool>,
    parent2: &Vec<bool>,
    k: usize,
    seed: Option<u64>,
) -> (Vec<bool>, Vec<bool>) {
    check_length(parent1, parent2);

    let mut prng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };


    let n = parent1.len();
    let mut indices = (0..n)
        .collect::<Vec<usize>>()
        .choose_multiple(&mut prng, k)
        .map(|&x| x)
        .collect::<Vec<usize>>();

    indices.sort();
    let (mut child1, mut child2) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let mut child_idx: usize = 0;
    for (i, &idx) in indices.iter().enumerate() {
        if i & 1 == 0 {
            while child_idx < idx {
                child1.push(parent1[child_idx]);
                child2.push(parent2[child_idx]);
                child_idx += 1;
            }
        } else {
            while child_idx < idx {
                child1.push(parent2[child_idx]);
                child2.push(parent1[child_idx]);
                child_idx += 1;
            }
        }
    }

    if child_idx < n {
        while child_idx < n {
            if k & 1 == 0 {
                child1.push(parent1[child_idx]);
                child2.push(parent2[child_idx]);
            } else {
                child1.push(parent2[child_idx]);
                child2.push(parent1[child_idx]);
            }
            child_idx += 1;
        }
    }

    return (child1, child2);
}
