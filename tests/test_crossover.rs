extern crate genx;

#[cfg(test)]
mod tests {
  use genx::crossover::{single_point_crossover, multi_point_crossover};

  #[test]
  fn test_single_point_crossover() {
    let parent1 = vec![false, true, false, false, true, true, true, false, true, false, false];
    let parent2 = vec![true, true, false, true, false, true, false, true, true, false, true];
    let childs = single_point_crossover(&parent1, &parent2, Some(42));
    assert_eq!(childs.0.len(), childs.1.len());
    let mut idx:usize = 0;
    for (i, x) in childs.0.iter().enumerate() {
      if *x != parent1[i] {
        idx = i;
        break;
      }
    }
    let mut expected_child = Vec::new();
    for i in 0..childs.0.len() {
      if i < idx {
        expected_child.push(parent1[i]);
      }
      else {
        expected_child.push(parent2[i]);
      }
    }
    assert_eq!(childs.0, expected_child);
    for (i, x) in expected_child.iter_mut().enumerate() {
      if i < idx {
        *x = parent2[i];
      }
      else {
        *x = parent1[i];
      }
    }
    assert_eq!(childs.1, expected_child);
  }

  #[test]
  fn test_multi_point_crossover(){
    let parent1 = vec![false, true, false, false, true, true, true, false, true, false, false];
    let parent2 = vec![true, true, false, true, false, true, false, true, true, false, true];
    let (child1, child2) = multi_point_crossover(&parent1, &parent2, 3, Some(42));
    assert_eq!(child1.len(), child2.len());
    assert_eq!(child1.len(), parent1.len());
    assert_eq!(child1, [false, true, false, true, false, true, true, false, true, false, true]);
    assert_eq!(child2, [true, true, false, false, true, true, false, true, true, false, false]);
  }
}
