extern crate genx;

#[cfg(test)]
mod tests {
  use genx::mutation::{flipping::flipping_mutation, inversion::inversion_mutation, polynomial::polynomial_mutation, random::random_mutation, scramble::scrable_mutation, swap::swap_mutation};

  #[test]
  fn test_inversion_mutation() {
    let mut individual = vec![false, true, false, false, true, true, true, false, true, true];
    let original_individual = individual.clone();
    inversion_mutation(&mut individual, Some(43));
    assert_eq!(individual, [false, true, false, false, false, false, false, true, true, true]);
    assert_ne!(original_individual, individual);
  }

  #[test]
  fn test_scramble_mutation() {
    let mut individual = vec![false, true, false, false, true, true, true, false, false, true, false];
    let original_individual = individual.clone();
    scrable_mutation(&mut individual, Some(43));
    assert_eq!(individual, [false, true, false, false, true, false, false, true, true, true, false]);
    assert_ne!(original_individual, individual);
  }

  #[test]
  fn test_swap_mutation_success() {
    let mut individual = vec![false, true, false, false, true, true, true, false, false, true, false];
    let original_individual = individual.clone();
    match swap_mutation(&mut individual, Some(43)) {
      Ok(_) => (),
      Err(error) => panic!("{:?}", error)
    };
    assert_eq!(individual, [false, true, false, false, true, true, true, false, true, false, false]);
    assert_ne!(original_individual, individual);
  }

  #[test]
  #[should_panic]
  fn test_swap_mutation_fail() {
    let mut individual = vec![false];
    match swap_mutation(&mut individual, Some(43)) {
      Ok(_) => (),
      Err(error) => panic!("{}", error)
    };
  }

  #[test]
  fn test_flipping_mutation_success() {
    let mut individual = vec![false, true, false, false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, false, false, true, false, true, true, false, false, true, false, true, false, false, true, true, true, false];
    let original_individual = individual.clone();
    match flipping_mutation(&mut individual, 0.05, Some(43)) {
      Ok(_) => (),
      Err(error) => panic!("{:?}", error)
    };
    assert_eq!(individual, [false, true, false, false, false, true, true, false, false, true, false, false, true, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, false, false, true, true, true, false]);
    assert_ne!(original_individual, individual);
  }

  #[test]
  #[should_panic]
  fn test_flipping_mutation() {
    let mut individual = vec![false, true, false, false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, false, false, true, false, true, true, false, false, true, false, true, false, false, true, true, true, false];
    let original_individual = individual.clone();
    match flipping_mutation(&mut individual, 1.05, Some(43)) {
      Ok(_) => (),
      Err(error) => panic!("{:?}", error)
    };
    assert_eq!(individual, [false, true, false, false, false, true, true, false, false, true, false, false, true, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, false, false, true, true, true, false]);
    assert_ne!(original_individual, individual);
  }

  #[test]
  fn test_random_mutation() {
    let individual = 29.11;
    let result = random_mutation(individual, 4.2, Some(43));
    assert_ne!(result, individual);
  }

  #[test]
  fn test_polynomial_mutation() {
    let individual = 29.11;
    let result = polynomial_mutation(individual, 4.2, Some(43));
    assert_ne!(result, individual);
  }
}
