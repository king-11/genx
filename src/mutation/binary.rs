use crate::{
  operator::{GeneticOperator, MutationOp},
  random::{Rng, random_cut_points},
};

/**
FlippingMutation mutation is a mutation only for binary encoded individuals. Given the [`mutation_probability`](./struct.FlippingMutation.html#structfield.mutation_probability) it iterates through the boolean vector of `genome`
and generates a random number between `0.0` and `1.0`. If the number is less than [`mutation_probability`](./struct.FlippingMutation.html#structfield.mutation_probability) then we _flip_ the value at that index else it remains the same.
*/
#[derive(Debug, Clone)]
pub struct FlippingMutation {
  pub mutation_probability: f64,
}

impl FlippingMutation {
/**
Returns a new Result with a FlippingMutation instance of given `mutation_probability`. If `mutation_probability` is less than `0.0` or greater than `1.0` then the Result will be an Err.
*/
  fn new(mutation_probability: f64) -> Result<Self, &'static str> {
      if mutation_probability < 0.0 || mutation_probability > 1.0 {
          return Err("mutation_probability should lie between 0.0 and 1.0 inclusive");
      }

      Ok(FlippingMutation {
          mutation_probability,
      })
  }

  fn mutation_probability(&self) -> f64 {
      self.mutation_probability
  }

  fn set_mutation_probability(&mut self, mutation_probability: f64) {
      self.mutation_probability = mutation_probability;
  }
}

impl GeneticOperator for FlippingMutation {
  fn name() -> String {
      String::from("Flipping-Mutation")
  }
}

impl MutationOp<Vec<bool>> for FlippingMutation {
  fn mutate<R>(&self, genome: &mut Vec<bool>, rng: &mut R)
  where
      R: Rng + Sized,
  {
      for val in genome.iter_mut() {
          let random_probability = rng.gen_range(0.0..1.0);
          if random_probability < self.mutation_probability {
              *val = !(*val);
          }
      }
  }
}

/**
Inversion mutation is a mutation only for binary encoded individuals. Given the `individual` it randomly generates two indices and then inverts the value between those indices of the individual.
*/
#[derive(Debug, Clone)]
pub struct InversionMutation {
}

impl GeneticOperator for InversionMutation {
  fn name() -> String {
      String::from("Inversion-Mutation")
  }
}

impl MutationOp<Vec<bool>> for InversionMutation {
  fn mutate<R>(&self, genome: &mut Vec<bool>, rng: &mut R)
  where
      R: Rng + Sized,
  {
    let genome_length = genome.len();
    let (locus1, locus2) = random_cut_points(rng, genome_length);

    let slice = &mut genome[locus1..=locus2];
    for x in slice.iter_mut() {
      *x = !*x;
    }
  }
}
