//! The `selection` module provides implementation of the
//! functions to select a set of individuals out of the total
//! population to do crossover for generating new individuals.
//!
//! The provided functions are organized in sub-modules
//! named after the utilized selection method:
//! * `random`
//! * `proportionate`
//! * `tournament`
//!
//! All the functions take in atleast two arguments a Vector of floating
//! point values that contains fitness values of individuals, and
//! number of individuals to select. Finally functions return indices of
//! selected individuals.
//!
//! You can read more about selection schemas and their working from the [wikipedia page](https://en.wikipedia.org/wiki/Selection_(genetic_algorithm))

pub mod random;

pub mod tournament;

pub mod proportionate;

// Re-exports
pub use self::random::RandomSelector;
pub use self::tournament::TournamentSelector;
pub use self::proportionate::*;
