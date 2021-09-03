//! The `mutation` module provides implementation of the
//! functions for various mutation schemes for binary encoded
//! and real value encoded individuals.
//!
//! The provided functions are organized in sub-modules
//! named after the utilized mutation schema.
//!
//! Available mutation schema for binary encoded
//! individuals are:
//! * `flipping`
//! * `inversion`
//! * `scramble`
//! * `swap`
//!
//! All the mutation functions for binary encoded schema
//! take in atleast an argument of mutable reference to
//! the boolean vector of individual to mutate. Hence they
//! change the actual boolean Vector.
//!
//! Only those functions where, there is a need to constrain
//! range of values that can be provided as argument will return
//! a `Result<(), &'static str>`. The returned value should be checked
//! in case `Err` was returned because of invalid argument values.
//!
//! Available mutation schema for real value encoded
//! individuals are:
//! * `random`
//! * `polynomial`
//!
//! All the mutation functions for real value encoded schema
//! take in the floating point value of individual and return
//! the mutated value.

pub mod flipping;

pub mod inversion;

pub mod polynomial;

pub mod random;

pub mod scramble;

pub mod swap;

// Re-exports
pub use self::flipping::flipping_mutation;
pub use self::inversion::inversion_mutation;
pub use self::polynomial::polynomial_mutation;
pub use self::random::random_mutation;
pub use self::scramble::scramble_mutation;
pub use self::swap::swap_mutation;
