//! The `encoding` module provides basic scheme of encoding
//! `genetic::Genotype`s.
//!
//! Most important encoding schemes are:
//! * binary encoding
//! * value encoding
//! * permutation encoding
//! * tree encoding
//!
//! To express which encoding scheme is used for a specific `genetic::Genotype`
//! a set of marker traits are defined:
//! * `BinaryEncoded`
//! * `ValueEncoded`
//! * `PermutationEncoded`
//! * `TreeEncoded`
//!
//! These marker traits are important for providing default implementations
//! for the `operator::CrossoverOp` and the `operator::MutationOp`. In order
//! to use any of the default operator implementation the `genetic::Genotype`
//! used for a genetic algorithm application must be marked with the
//! appropriate encoding trait. If an application is defining its own crossover
//! and mutation operators then using these marker traits is optional.

use crate::genetic::Genotype;
use std::{fmt::Debug, collections::HashSet};

/// Marker trait for declaring a `genetic::Genotype` as binary encoded.
pub trait BinaryEncoded {}

/// Marker trait for declaring a `genetic::Genotype` as value encoded.
pub trait ValueEncoded {}

/// Marker trait for declaring a permutation encoded `genetic::Genotype`.
pub trait PermutationEncoded {
  // Checks if all the values are presnet
  // in the permutation encoding from 0 to n-1
  // where n is the length of the phenotype.
  fn check_continuous(&self) -> bool;
}

/// Implementation of a genotype using `Vec`.
impl<V> Genotype for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    type Dna = V;
}

/// Implementation of binary encoded `genetic::Genotype`
/// using `Vec<bool>`.
impl BinaryEncoded for Vec<bool> {}

/// Implementation of a value encoded `genetic::Genotype`.
/// using `Vec`.
impl<V> ValueEncoded for Vec<V> {}

/// Implementation of a permutation encoded `genetic::Genotype`
/// using `Vec<usize>`.
impl PermutationEncoded for Vec<usize> {
  fn check_continuous(&self) -> bool {
    let n = self.len();
    let mut set:HashSet<usize> = HashSet::new();
    for x in self.iter() {
      set.insert(*x);
      if *x >= n {
        return false;
      }
    }

    set.len() == self.len()
  }
}
