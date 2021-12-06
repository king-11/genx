//! Crossover is one of the prominent operators used in genetic algorithms.
//! Crossover process is vital in generating new chromosomes by combing two or more parent
//! chromosomes with the hope that they create new and efficient chromosomes.
//! Crossover occurs after selection of pairs of parent chromosomes and helps in
//! exchanging information between parents to create children.
//!
//! During crossover the
//! parent chromosomes are taken in pair and their genes are exchanged in certain order
//! to obtain off spring. These offspring become next generation parent chromosomes.
//! It is performed by exchanging alleles between two selected parent
//! chromosomes in order to explore new solution space.
//!
//! All the crossover functions take in atleast two arguments
//! which are the two parent from which two offsprings are generated.
//!
//! Available crossover functions for binary encoded
//! individuals are:
//! * `single_point`
//! * `multi_point`
//! * `uniform`
//! * `shuffle`
//!
//! Available crossover functions for real encoded
//! individuals are:
//! * `linear`
//! * `blend`
//! * `simulated_binary`
//!
//! Above all real encoded crossover techniques take in one additional parameter
//! that determines the shift in values between the parents and offsprings.
//!
//! Available crossover functions for order encoded
//! individuals are:
//! * `cycle`
//! * `order`
//! * `partially_mapped`
//! * `uniform_partially_mapped`
//!
//! For order encoded individuals the crossover function will panic in case
//! of invalid order or missing elements. So values should be in range of
//! 0..n-1 where n is length of the order encoded individual.
//!
//! You can read more about selection schemas and their working from the [research paper](http://ictactjournals.in/paper/IJSC_V6_I1_paper_4_pp_1083_1092.pdf)
use std::collections::HashSet;

pub mod generic;

pub mod uniform;

pub mod order;

pub mod cycle;

pub mod linear;

pub mod blend;

pub mod simulated_binary;

pub use self::generic::*;

pub use self::uniform::uniform_crossover;

pub use self::order::*;

pub use self::cycle::cycle_crossover;

pub use self::linear::linear_crossover;

pub use self::blend::blend_crossover;

pub use self::simulated_binary::simulated_binary_crossover;

fn check_continuous(vec: &Vec<usize>) -> bool {
  let n = vec.len();
  let mut set:HashSet<usize> = HashSet::new();
  for x in vec.iter() {
    set.insert(*x);
    if *x >= n {
      return false;
    }
  }

  set.len() == vec.len()
}

fn check_length<T>(parent1 : &Vec<T>, parent2 : &Vec<T>) {
  if parent1.len() != parent2.len() {
    panic!("Vectors must be the same length");
  }
}
