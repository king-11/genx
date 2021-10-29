use std::collections::{HashMap, HashSet};

use itertools::multizip;

use super::{check_continuous, check_length};

pub fn cycle_crossover(parent1: &Vec<usize>, parent2: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
  check_length(parent1, parent2);

  if !check_continuous(parent1) || !check_continuous(parent2) {
    panic!("Vectors must contain continuous unique values");
  };

  let (mut map1, mut map2) = (HashMap::new(), HashMap::new());

  for (idx, (&val1, &val2)) in multizip((parent1, parent2)).enumerate() {
    map1.insert(val1, idx);
    map2.insert(val2, idx);
  }

  let (mut child1, mut child2) = (parent2.clone(), parent1.clone());
  let mut set:HashSet<usize> = HashSet::new();

  let mut current_val:(usize, usize) = (parent1[0], 0);
  while !set.get(&current_val.0).is_some() {
    set.insert(current_val.0);
    child1[current_val.1] = parent1[current_val.1];
    current_val = (parent2[current_val.1], map1[&parent2[current_val.1]])
  }

  set.clear();
  current_val = (parent2[0], 0);
  while !set.get(&current_val.0).is_some() {
    set.insert(current_val.0);
    child2[current_val.1] = parent2[current_val.1];
    current_val = (parent1[current_val.1], map2[&parent1[current_val.1]])
  }
  set.clear();

  (child1, child2)
}
