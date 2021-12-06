use crate::{
  operator::{GeneticOperator, MutationOp},
  random::{random_cut_points, Rng},
};
use rand::seq::SliceRandom;
use std::fmt::Debug;

/**
Scramble mutation is a mutation only for vector encoded individuals.Given the `individual` it randomly generates two indices and then shuffles the values between those two indices.
*/
#[derive(Debug, Clone)]
pub struct ScrambleMutation {}

impl GeneticOperator for ScrambleMutation {
  fn name() -> String {
      String::from("Scramble-Mutation")
  }
}

impl<V> MutationOp<Vec<V>> for ScrambleMutation
where
  V: Clone + Debug + PartialEq + Send + Sync,
{
  fn mutate<R>(&self, genome: &mut Vec<V>, rng: &mut R)
  where
      R: Rng + Sized,
  {
      let genome_length = genome.len();
      let (locus1, locus2) = random_cut_points(rng, genome_length);

      let slice = &mut genome[locus1..=locus2];
      slice.shuffle(rng);
  }
}

/**
 Swap mutation is a mutation only for vector encoded genomes. Given the `individual` it randomly generates two indices and then swaps the value at those two indices. Based on the [`mutation_rate`](./struct.SwapMutation.html#structfield.mutation_rate) it decides how many mutations to apply.
*/
#[derive(Debug, Clone)]
pub struct SwapMutation {
    pub mutation_rate: f64,
}

impl SwapMutation {
    pub fn new(mutation_rate: f64) -> Self {
        Self { mutation_rate }
    }

    pub fn mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    pub fn set_mutation_rate(&mut self, mutation_rate: f64) {
        self.mutation_rate = mutation_rate;
    }
}

impl GeneticOperator for SwapMutation {
    fn name() -> String {
        String::from("Swap-Mutation")
    }
}

impl<V> MutationOp<Vec<V>> for SwapMutation
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    fn mutate<R>(&self, genome: &mut Vec<V>, rng: &mut R)
    where
        R: Rng + Sized,
    {
        let genome_length = genome.len();
        let num_mutations =
            ((genome_length as f64 * self.mutation_rate) + rng.gen::<f64>()).floor() as usize;
        for _ in 0..num_mutations {
            let (locus1, locus2) = random_cut_points(rng, genome_length);
            genome.swap(locus1, locus2);
        }
    }
}
