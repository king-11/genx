use std::{ops::{Add, Mul}, fmt::Debug};

use crate::{
    operator::{GeneticOperator, MutationOp},
    prelude::Genotype,
    random::Rng,
};
/**
Random mutation is a mutation only for real encoded individuals. Given the `individual` and `perturbation_factor` it generates a random value in range `0.0..1.0` which is then used to linearly mutate the individual.

 _Note: A large value for `perturbation_factor` will make stark mutation for all values._
*/
#[derive(Debug, Clone)]
pub struct RandomMutation<G> {
    perturbation_factor: G,
}

impl<G> RandomMutation<G> {
    pub fn new(perturbation_factor: G) -> Self {
        RandomMutation {
            perturbation_factor,
        }
    }

    pub fn perturbation_factor(&self) -> &G {
        &self.perturbation_factor
    }

    pub fn set_perturbation_factor(&mut self, perturbation_factor: G) {
        self.perturbation_factor = perturbation_factor;
    }
}

impl<G> GeneticOperator for RandomMutation<G>
where
    G: Genotype,
{
    fn name() -> String {
        String::from("Random-Mutation")
    }
}

impl<G> MutationOp<G> for RandomMutation<G>
where
    G: Genotype + From<f64> + Add<Output = G> + Mul<Output = G> + Copy,
{
    fn mutate<R>(&self, genome: &mut G, rng: &mut R)
    where
        R: Rng + Sized,
    {
        let random_value = rng.gen::<f64>();
        let val: G = (2.0 * random_value - 1.0).into();
        *genome = *genome + val * (self.perturbation_factor);
    }
}

/**
 Polynomial mutation is a mutation only for real encoded individuals.
 Given the `individual`, `distribution_index` and `max_perturbation` it generates a closure which is an exponential polynomial function using the `distribution_index`. Given a randomly generated value in range `0.0..1.0` to the closure it returns the `perturbation_factor`.
 */
#[derive(Clone, Debug)]
pub struct PolynomialMutation {
    distribution_index: f64,
    max_perturbation: f64,
}

impl PolynomialMutation {
    pub fn new(distribution_index: f64, max_perturbation: f64) -> Self {
        Self {
            distribution_index,
            max_perturbation,
        }
    }

    /**
      A large value for [`distribution_index`] will make stark mutation for only a few values whereas a small [`distribution_index`] will cause uniform mutation for all randomly generated values (undesirable).
    */
    pub fn distribution_index(&self) -> f64 {
        self.distribution_index
    }

    pub fn set_distribution_index(&mut self, distribution_index: f64) {
        self.distribution_index = distribution_index;
    }

    pub fn max_perturbation(&self) -> f64 {
        self.max_perturbation
    }

    pub fn set_max_perturbation(&mut self, max_perturbation: f64) {
        self.max_perturbation = max_perturbation;
    }
}

impl GeneticOperator for PolynomialMutation {
    fn name() -> String {
        String::from("Polynomial-Mutation")
    }
}

impl<G> MutationOp<G> for PolynomialMutation
where
    G: Genotype + From<f64> + Add<Output = G> + Copy,
{
    fn mutate<R>(&self, genome: &mut G, rng: &mut R)
    where
        R: Rng + Sized,
    {
        let random_value = rng.gen::<f64>();
        let calculate_perturbation_factor = |random_value: f64| -> f64 {
            if random_value < 0.5 {
                return (2.0 * random_value).powf(1.0 / (self.distribution_index + 1.0)) - 1.0;
            }
            return 1.0 - (2.0 * (1.0 - random_value).powf(1.0 / (self.distribution_index + 1.0)));
        };

        *genome =
            *genome + (calculate_perturbation_factor(random_value) * self.max_perturbation).into()
    }
}

trait Tof64 {}
