use crate::{
    algorithm::EvaluatedPopulation,
    operator::{GeneticOperator, ScalingOp},
    prelude::{Fitness, Genotype}, genetic::AsScalar,
};

#[derive(Debug, Clone)]
pub struct LinearScaler {
    scalling_factor: f64,
}

impl LinearScaler {
    pub fn new(scalling_factor: f64) -> LinearScaler {
        LinearScaler { scalling_factor }
    }

    pub fn scaling_factor(&self) -> f64 {
        self.scalling_factor
    }

    pub fn set_scaling_factor(&mut self, scalling_factor: f64) {
        self.scalling_factor = scalling_factor;
    }
}

impl GeneticOperator for LinearScaler {
    fn name() -> String {
        String::from("Linear-Scaling")
    }
}

impl<G, F> ScalingOp<G, F> for LinearScaler
where
    G: Genotype,
    F: Fitness + AsScalar,
{
    fn scale(&self, population: EvaluatedPopulation<G, F>) -> EvaluatedPopulation<G, F> {
        let individuals = population.individuals();

        let min_fit = population.lowest_fitness().as_scalar();
        let max_fit = population.highest_fitness().as_scalar();
        let avg_fit = population.average_fitness().as_scalar();

        let mut a =
        (avg_fit * (self.scaling_factor() - 1.0)) / (max_fit - avg_fit);
        let mut b = (avg_fit * (max_fit - self.scaling_factor() * avg_fit))
        / (max_fit - avg_fit);

        if min_fit <= -1.0 * b / a {
            a = avg_fit / (avg_fit - min_fit);
            b = -1.0 * min_fit * avg_fit / (avg_fit - min_fit);
        };

        let linear_functions = |x: f64| {
            a * x + b
        };

        let fitness_values = population.fitness_values().iter().map(|x| {
            linear_functions(x.as_scalar()).into()
        }).collect::<Vec<F>>();

        let min_fit = fitness_values.iter().min().unwrap();

        EvaluatedPopulation::new(individuals, fitness_values.to_vec(), (max_fit*self.scaling_factor()).into(), min_fit.clone(), avg_fit.into())
    }
}
