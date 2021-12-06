use crate::{
    algorithm::EvaluatedPopulation,
    genetic::AsScalar,
    operator::{GeneticOperator, ScalingOp},
    prelude::{Fitness, Genotype},
};

#[derive(Clone, Debug)]
pub struct SigmaScaler {
    scaling_factor: f64,
}

impl SigmaScaler {
    pub fn new(scaling_factor: f64) -> SigmaScaler {
        SigmaScaler { scaling_factor }
    }

    pub fn scaling_factor(&self) -> f64 {
        self.scaling_factor
    }

    pub fn set_scaling_factor(&mut self, scaling_factor: f64) {
        self.scaling_factor = scaling_factor;
    }
}

impl GeneticOperator for SigmaScaler {
    fn name() -> String {
        String::from("Sigma-Scaler")
    }
}

impl<G, F> ScalingOp<G, F> for SigmaScaler
where
    G: Genotype,
    F: Fitness + AsScalar,
{
    fn scale(&self, population: EvaluatedPopulation<G, F>) -> EvaluatedPopulation<G, F> {
        let individuals = population.individuals();
        let average_fitness = population.average_fitness().as_scalar();
        let mut fitness_values = population
            .fitness_values()
            .iter()
            .map(|x| x.as_scalar())
            .collect::<Vec<f64>>();

        let standard_deviation = fitness_values
            .iter()
            .map(|x| (x - average_fitness).powi(2))
            .sum::<f64>();
        let standard_deviation = (standard_deviation / (fitness_values.len() as f64)).sqrt();
        let worst_fitness = average_fitness - standard_deviation * self.scaling_factor();

        let fitness_values = fitness_values
            .iter()
            .map(|x| {
                if x < &worst_fitness {
                    0.0.into()
                } else {
                    (x - worst_fitness).into()
                }
            })
            .collect::<Vec<F>>();

        let max_fit = fitness_values.iter().max().unwrap();
        EvaluatedPopulation::new(
            individuals,
            fitness_values.clone(),
            max_fit.clone(),
            0.0.into(),
            population.average_fitness().clone(),
        )
    }
}
