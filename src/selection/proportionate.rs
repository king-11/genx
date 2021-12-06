//! The `proportionate` module provides `operator::SelectionOp`s that implement
//! stochastic fitness proportionate selection strategies. Individuals are
//! randomly selected. Individuals with a higher `genetic::Fitness` value are
//! having a higher probability to be selected.
//!
//! How is this achieved?
//! In fitness proportionate selection each individual gets assigned a weight
//! that is equal to its fitness value plus the sum of fitness values of all
//! individuals in the list before him (cumulative weight distribution). Then
//! a uniform random number between 0 and the sum of all weights is used to
//! select a candidate.
//!
//! The provided **fitness proportionate selection** operators are:
//! * `RouletteWheelSelector` - no bias - does not guarantee minimal spread.
//! * `RankWheelSelection` - no bias - does guarantee moderate spread.
//! * `UniversalSamplingSelector` - no bias - minimal spread.

use std::{cmp::Ordering, f64};

use crate::{
    genetic::{AsScalar, Parents},
    operator::{GeneticOperator, SelectionOp},
    prelude::{Fitness, Genotype},
    random::{random_probability, WeightedDistribution, Rng}, algorithm::EvaluatedPopulation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RouletteWheelSelector {
    selection_ratio: f64,
}

impl RouletteWheelSelector {
    pub fn new(selection_ratio: f64) -> Self {
        RouletteWheelSelector {
            selection_ratio,
        }
    }

    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    pub fn set_selection_ratio(&mut self, selection_ratio: f64) {
        self.selection_ratio = selection_ratio;
    }
}

impl GeneticOperator for RouletteWheelSelector {
    fn name() -> String {
        String::from("RouletteWheel-Selector")
    }
}

impl<G, F> SelectionOp<G, F> for RouletteWheelSelector
where
    G: Genotype,
    F: Fitness + AsScalar,
{
    fn select_from<R>(
        &self,
        population: &EvaluatedPopulation<G, F>,
        rng: &mut R,
    ) -> Parents<G>
    where
        R: Rng + Sized,
    {
        let individuals = population.individuals();
        let num_parents_to_select =
            (individuals.len() as f64 * self.selection_ratio + 0.5).floor() as usize;
        let mut parents = Vec::with_capacity(num_parents_to_select);
        let weighted_distribution =
            WeightedDistribution::from_scalar_values(population.fitness_values());
        for _ in 0..num_parents_to_select {
            let random = random_probability(rng) * weighted_distribution.sum();
            let selected = weighted_distribution.select(random);
            parents.push(individuals[selected].clone());
        }
        parents
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct RankWheelSelector {
    selection_ratio: f64,
}

impl RankWheelSelector {
    pub fn new(selection_ratio: f64) -> Self {
        RankWheelSelector {
            selection_ratio,
        }
    }

    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    pub fn set_selection_ratio(&mut self, selection_ratio: f64) {
        self.selection_ratio = selection_ratio;
    }
}

impl GeneticOperator for RankWheelSelector {
    fn name() -> String {
        String::from("RankWheel-Selector")
    }
}

impl<G, F> SelectionOp<G, F> for RankWheelSelector
where
    G: Genotype,
    F: Fitness + AsScalar + Copy,
{
    fn select_from<R>(
        &self,
        population: &EvaluatedPopulation<G, F>,
        rng: &mut R,
    ) -> Parents<G>
    where
        R: rand::Rng + Sized,
    {
        let individuals = population.individuals();
        let num_parents_to_select =
            (individuals.len() as f64 * self.selection_ratio + 0.5).floor() as usize;
        let mut parents = Vec::with_capacity(num_parents_to_select);

        let mut fitness_values_with_index : Vec<(F, usize)> = Vec::new();
        for (i, &value) in population.fitness_values().iter().enumerate() {
            fitness_values_with_index.push((value, i));
        };
        fitness_values_with_index.sort_unstable_by(|&(a, _), (b, _)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let mut ranks:Vec<f64> = vec![1.0];
        for (i, _) in fitness_values_with_index.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if fitness_values_with_index[i].0 == fitness_values_with_index[i-1].0 {
                ranks.push(ranks[i-1]);
            }
            else {
                ranks.push(ranks[i-1] + 1.0);
            }
        };

        let mut weights = ranks.iter().map(|&rank| rank.into()).collect::<Vec<F>>();
        for (idx, &(_, i)) in fitness_values_with_index.iter().enumerate() {
          weights[i] = ranks[idx].into();
        }

        let weighted_distribution =
            WeightedDistribution::from_scalar_values(&weights);
        for _ in 0..num_parents_to_select {
            let random = random_probability(rng) * weighted_distribution.sum();
            let selected = weighted_distribution.select(random);
            parents.push(individuals[selected].clone());
        }
        parents
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StochasticUniversalSelector {
    selection_ratio: f64,
}

impl StochasticUniversalSelector {
    pub fn new(selection_ratio: f64) -> Self {
        StochasticUniversalSelector {
            selection_ratio,
        }
    }

    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    pub fn set_selection_ratio(&mut self, selection_ratio: f64) {
        self.selection_ratio = selection_ratio;
    }
}

impl GeneticOperator for StochasticUniversalSelector {
    fn name() -> String {
        String::from("StochasticUniversal-Selector")
    }
}

impl<G, F> SelectionOp<G, F> for StochasticUniversalSelector
where
    G: Genotype,
    F: Fitness + AsScalar + Copy,
{
    fn select_from<R>(
        &self,
        population: &EvaluatedPopulation<G, F>,
        rng: &mut R,
    ) -> Parents<G>
    where
        R: rand::Rng + Sized,
    {
        let individuals = population.individuals();
        let num_parents_to_select =
            (individuals.len() as f64 * self.selection_ratio + 0.5).floor() as usize;
        let mut parents = Vec::with_capacity(num_parents_to_select);

        let weighted_distribution =
            WeightedDistribution::from_scalar_values(&population.fitness_values());
        let fitness_step = weighted_distribution.sum() / num_parents_to_select as f64;
        let random_offset = rng.gen_range(0.0..fitness_step);
        for i in 0..num_parents_to_select {
            let random = i as f64 * fitness_step + random_offset;
            let selected = weighted_distribution.select(random);
            parents.push(individuals[selected].clone());
        }
        parents
    }
}
