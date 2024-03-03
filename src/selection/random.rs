use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::{
    algorithm::EvaluatedPopulation,
    genetic::{AsScalar, Parents},
    operator::{GeneticOperator, SelectionOp},
    prelude::{Fitness, Genotype},
};

/**
Random Selection is the simplest form of selection which randomly generates an index from the provided `parents` vector and that is the selection. Same procedure is done until we have reached `selection_ratio`.
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RandomSelector {
    selection_ration: f64,
}

impl RandomSelector {
    pub fn new(selection_ration: f64) -> Self {
        Self { selection_ration }
    }

    pub fn selection_ratio(&self) -> f64 {
        self.selection_ration
    }

    pub fn set_selection_ratio(&mut self, selection_ration: f64) {
        self.selection_ration = selection_ration;
    }
}

impl GeneticOperator for RandomSelector {
    fn name() -> String {
        String::from("Random-Selector")
    }
}

impl<G, F> SelectionOp<G, F> for RandomSelector
where
    G: Genotype,
    F: Fitness + AsScalar,
{
    fn select_from<R>(&self, population: &EvaluatedPopulation<G, F>, rng: &mut R) -> Parents<G>
    where
        R: Rng + Sized,
    {
        let individuals = population.individuals();
        let num_parents_to_select =
            (individuals.len() as f64 * self.selection_ratio() + 0.5).floor() as usize;
        let mut parents = Vec::with_capacity(num_parents_to_select);
        let uniform = Uniform::from(0..individuals.len());
        for _ in 0..num_parents_to_select {
            parents.push(individuals[uniform.sample(rng)].clone());
        }
        parents
    }
}
