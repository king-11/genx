use std::collections::HashSet;
use super::{check_length, check_continuous};
use rand::{SeedableRng, rngs::StdRng, prelude::IteratorRandom};

pub fn davis_order_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, seed: Option<u64>) -> (Vec<usize>, Vec<usize>) {
  check_length(parent1, parent2);

  if !check_continuous(parent1) || !check_continuous(parent2) {
    panic!("Vectors must contain continuous unique values");
  }

  let n = parent1.len();
  let mut prng = match seed {
    Some(seed) => StdRng::seed_from_u64(seed),
    None => StdRng::from_entropy(),
  };

  let mut selected = (0..n).map(|x| x).choose_multiple(&mut prng, 2);
  selected.sort();

  let (mut child1, mut child2) = (parent1.clone(), parent2.clone());
  let mut set1: HashSet<usize> = HashSet::new();
  let mut set2: HashSet<usize> = HashSet::new();

  for i in selected[0]..=selected[1] {
    set1.insert(child1[i]);
    set2.insert(child2[i]);
  }

  fill_missing_davis(&mut child1, &mut set1, parent2, (selected[0], selected[1]));
  fill_missing_davis(&mut child2, &mut set2, parent1, (selected[0], selected[1]));
  (child1, child2)
}

fn fill_missing_davis(child: &mut Vec<usize>, has: &mut HashSet<usize>, buffer: &Vec<usize>, skip: (usize, usize)) {
  let mut i = 0;
  let n = child.len();
  for val in buffer.iter() {
    if i == skip.0 {
      i = skip.1 + 1;
    }
    if i == n {
      return;
    }
    if !has.contains(val) {
      child[i] = *val;
      i += 1;
      has.insert(*val);
    }
  }
}
