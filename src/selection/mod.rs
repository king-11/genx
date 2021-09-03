//! The `selection` module provides implementation of the
//! functions to select a set of individuals out of the total
//! population to do crossover for generating new individuals.
//!
//! The provided functions are organized in sub-modules
//! named after the utilized selection method:
//! * `random`
//! * `rank`
//! * `roulette_wheel`
//! * `steady_state`
//! * `stochastic_universal`
//! * `tournament`
//!
//! All the functions take in atleast two arguments a Vector of floating
//! point values that contains fitness values of individuals, and
//! number of individuals to select. Finally functions return indices of
//! selected individuals.
//!
//! You can read more about selection schemas and their working from the [wikipedia page](https://en.wikipedia.org/wiki/Selection_(genetic_algorithm))

pub mod random;

pub mod rank;

pub mod roulette_wheel;

pub mod steady_state;

pub mod stochastic_universal;

pub mod tournament;

// Re-exports
pub use self::random::random_selection;
pub use self::rank::rank_selection;
pub use self::roulette_wheel::roulette_wheel_selection;
pub use self::steady_state::steady_state_selection;
pub use self::stochastic_universal::stochastic_universal_selection;
pub use self::tournament::tournament_selection;
