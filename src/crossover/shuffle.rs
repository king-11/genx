use rand::{SeedableRng, prelude::SliceRandom, rngs::StdRng};

use super::{check_length, single_point_crossover};

pub fn shuffle_crossover(parent1: &Vec<bool>, parent2: &Vec<bool>, seed: Option<u64>) -> (Vec<bool>, Vec<bool>) {
  check_length(parent1, parent2);

  let mut prng = match seed {
    Some(seed) => StdRng::seed_from_u64(seed),
    None => StdRng::from_entropy(),
  };


  let mut indices = (0..parent1.len()).collect::<Vec<usize>>();
  indices.shuffle(&mut prng);
  let shuffled_parent1 = indices.iter().map(|&i| parent1[i]).collect::<Vec<bool>>();
  let shuffled_parent2 = indices.iter().map(|&i| parent2[i]).collect::<Vec<bool>>();

  let (shuffled_child1, shuffled_child2) = single_point_crossover(&shuffled_parent1, &shuffled_parent2, seed);

  let mut child1 = (0..parent1.len()).map(|_| false).collect::<Vec<bool>>();
  let mut child2 = (0..parent2.len()).map(|_| false).collect::<Vec<bool>>();

  for (idx, &index) in indices.iter().enumerate() {
    child1[index] = shuffled_child1[idx];
    child2[index] = shuffled_child2[idx];
  };

  return (child1, child2);
}
