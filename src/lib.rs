//! **Genx** provides modular building blocks to run simulations of optimization
//! and search problems using [Genetic Algorithms](https://en.wikipedia.org/wiki/Genetic_algorithm) (GA).
//!
//! The vision for genx is to be a **flexible** and greatly **extensible** library for implementing genetic
//! algorithm applications. genx is written in Rust. The library's API utilizes _functional
//! programming_ paradigm and exposes it's API in that manner only.
//!
//! The implementation is split into __building blocks__ which are all represented
//! by traits. This crate provides most common and probably all possible implementation
//! for all building blocks. So it can be used for many problems out of the box.
//!
//! ## Basic Example
//!
//! ### Selection
//!
//! Here's a trivial example that returns back individual selected using [stochastic universal sampling](https://en.wikipedia.org/wiki/Stochastic_universal_sampling)
//!
//! ```rust
//! use genx::selection::stochastic_universal::stochastic_universal_selection;
//!
//! let num_parents:usize = 10;
//! let fitness_values = vec![2.4,8.4,3.2,9.4,9.0,11.0,4.5,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7];
//!
//! let result = stochastic_universal_selection(&fitness_values, num_parents, None)
//!                 .iter()
//!                 .map(|&a| fitness_values[a])
//!                 .collect::<Vec<f32>>();
//! ```
//!
//! `stochastic_universal_selection` takes in the `fitness_value` vector, number of parents it needs to select and a seed value
//! which is `Option<u64>`. It returns back the indices of selected individuals which we map to actual fitness values.
//!
//! ### Mutation
//!
//! Mutation function takes in a single individual, distribution index and returns in the mutated individual using [polynomial mutation](https://www.iitk.ac.in/kangal/papers/k2012016.pdf) for real valued individual.
//!
//! ```rust
//! use genx::mutation::polynomial::polynomial_mutation;
//! let individual = 29.11;
//! let result = polynomial_mutation(individual, 4.2, 4.0, None);
//! ```
//!
//! The returned value may or may not be equal as is mutated based on a randomly generated value which
//! for deterministic results can be seeded.

//! ## Building Blocks
//!
//! The genetic algorithm needs a population that it evolves with each iteration.
//! A population contains a number of individuals. Each individual represents a possible
//! candidate solution for an optimization problem for which the best solution is searched for.
//!
//! ### Steps in Genetic Algorithm
//!
//! A Genetic Algorithm proceeds through following operations:
//! - **Encoding**: Binary, Real Values, Order, Tree, etc.
//! - **Selection**: Selecting individuals after fitness evaluation.
//! - **Crossover**: Creating new individuals from selected pool of individuals.
//! - **Mutation**: Making stark changes in generated individual for diversification.
//! - **Convergence**: Test for goal accomplishment or convergence.
//!
//! ### Available Building Blocks
//!
//! The building blocks available in the crate (defined as traits) are:
//! - **[`selection`]**
//! - **[`mutation`]**
//! - **[`crossover`]**
//! - **[`scaling`]**
//!
//! This crate provides multiple implementations for each one of those operators.
//! So one can experiment with combining the different implementations to compose
//! the best algorithm for a specific search or optimization problem.

//! ## Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! genx = "0.3.3"
//! ```
//! If you are not using Rust 2018 edition add this to your crate root:
//! ```rust
//! extern crate genx;
//! ```

//! ## Why Genetic Algorithms
//!
//! Genetic Algorithms are at the core of [soft computing](https://en.wikipedia.org/wiki/Soft_computing) which is a branch of
//! computing that comes to rescue when problem at hand is not feasible to be solved using [hard computing](http://www2.cs.uh.edu/~ceick/6367/Soft-Computing.pdf).
//! There are several advantages of genetic algorithms:
//!
//! - Algorithms are adaptive and can adjust to the change of dynamic environment​
//! - The approach makes use of approximate solutions to solve problems that may be either unsolvable or too time-consuming to solve with current hardware.​
//! - Imprecise but usable solutions to complex computational problems allowing researchers to approach some problems that traditional computing cannot process.​

pub mod mutation;

pub mod selection;

pub mod crossover;

pub mod scaling;
