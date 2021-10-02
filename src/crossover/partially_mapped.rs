use std::collections::HashMap;

use super::{check_continuous, check_length};

use rand::{SeedableRng, prelude::IteratorRandom, rngs::StdRng};

pub fn partially_mapped_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>, seed: Option<u64>) -> (Vec<usize>, Vec<usize>) {
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
  let mut mapping1:HashMap<usize, usize> = HashMap::new();
  let mut mapping2:HashMap<usize, usize> = HashMap::new();
  let (mut child1, mut child2) = (Vec::with_capacity(n), Vec::with_capacity(n));
  for i in 0..n {
    if i < selected[0] || i > selected[1] {
      child1.push(parent1[i]);
      child2.push(parent2[i]);
    }
    else {
      child1.push(parent2[i]);
      child2.push(parent1[i]);
      mapping1.insert(parent2[i], parent1[i]);
      mapping2.insert(parent1[i], parent2[i]);
    }
  }

  mapping1 = simplify_mapping(mapping1);
  mapping2 = simplify_mapping(mapping2);

  for i in 0..n {
    if i < selected[0] || i > selected[1] {
      child1[i] = mapping1.get(&child1[i]).unwrap_or(&child1[i]).clone();
      child2[i] = mapping2.get(&child2[i]).unwrap_or(&child2[i]).clone();
    }
  }

  (child1, child2)
}

fn simplify_mapping(mapping: HashMap<usize, usize>) -> HashMap<usize, usize> {
  let mut simplified = HashMap::new();
  for (key, val) in mapping.iter() {
    let mut temp_val = val;
    while mapping.contains_key(temp_val) {
      temp_val = mapping.get(temp_val).unwrap();
    }
    simplified.insert(*key, *temp_val);
  }

  simplified
}
