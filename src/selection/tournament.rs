use rand::{seq::SliceRandom, Rng};
use std::cmp::min;

use crate::{
    algorithm::EvaluatedPopulation,
    genetic::{AsScalar, Parents},
    operator::{GeneticOperator, SelectionOp},
    prelude::{Fitness, Genotype},
};

/**
## Description
Tournament Selection is a randomized sorting based selection method. We conduct `num_parents` tournaments where we randomly select `tournament_size` individuals. Out of those selected for a particular tournament, the fittest one is added to the vector of selected indices.

_Note: If `tournament_size` is greater than length of `fitness_values` vector (n), then overall fittest individual gets selected from all the tournaments._
*/
#[derive(Clone, Debug, PartialEq)]
pub struct TournamentSelector {
    selection_ratio: f64,
    tournament_size: usize,
}

impl TournamentSelector {
    pub fn new(selection_ratio: f64, tournament_size: usize) -> Self {
        TournamentSelector {
            selection_ratio,
            tournament_size,
        }
    }

    pub fn selection_ratio(&self) -> f64 {
        self.selection_ratio
    }

    pub fn set_selection_ratio(&mut self, selection_ratio: f64) {
        self.selection_ratio = selection_ratio;
    }

    pub fn tournament_size(&self) -> usize {
        self.tournament_size
    }

    pub fn set_tournament_size(&mut self, tournament_size: usize) {
        self.tournament_size = tournament_size;
    }
}

impl GeneticOperator for TournamentSelector {
    fn name() -> String {
        String::from("Tournament-Selector")
    }
}

impl<G, F> SelectionOp<G, F> for TournamentSelector
where
    G: Genotype + Ord,
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
        for _ in 0..num_parents_to_select {
            let mut tournament = individuals
                .choose_multiple(rng, min(self.tournament_size(), individuals.len()))
                .collect::<Vec<_>>();
            tournament.sort();
            parents.push(tournament[0].clone());
        }
        parents
    }
}
