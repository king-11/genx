use rand::{Rng, prelude::SliceRandom};

use crate::{
    genetic::{Children, Parents},
    operator::{CrossoverOp, GeneticOperator},
    prelude::Genotype,
    random::random_n_cut_points,
};

use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct SinglePointCrossBreeder {}

impl SinglePointCrossBreeder {
    pub fn new() -> Self {
        SinglePointCrossBreeder {}
    }
}

impl GeneticOperator for SinglePointCrossBreeder {
    fn name() -> String {
        "Single-Point-Cross-Breeder".to_string()
    }
}

impl<G> CrossoverOp<G> for SinglePointCrossBreeder
where
    G: Genotype + MultiPointCrossover,
{
    fn crossover<R>(&self, parents: &Parents<G>, rng: &mut R) -> Children<G>
    where
        R: Rng + Sized,
    {
        MultiPointCrossover::crossover(&parents, 1, rng)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MultiPointCrossBreeder {
    num_cut_points: usize,
}

impl MultiPointCrossBreeder {
    pub fn new(num_cut_points: usize) -> Self {
        MultiPointCrossBreeder { num_cut_points }
    }

    /// Returns the number of cut points used by this operator.
    pub fn num_cut_points(&self) -> usize {
        self.num_cut_points
    }

    /// Sets the number of cut points used by this operator to the given value.
    pub fn set_num_cut_points(&mut self, value: usize) {
        self.num_cut_points = value;
    }
}

impl GeneticOperator for MultiPointCrossBreeder {
    fn name() -> String {
        String::from("Multi-Point-Cross-Breeder")
    }
}

impl<G> CrossoverOp<G> for MultiPointCrossBreeder
where
    G: Genotype + MultiPointCrossover,
{
    fn crossover<R>(&self, parents: &Parents<G>, rng: &mut R) -> Children<G>
    where
        R: Rng + Sized,
    {
        MultiPointCrossover::crossover(parents, self.num_cut_points, rng)
    }
}

pub trait MultiPointCrossover: Genotype {
    type Dna;

    fn crossover<R>(parents: &Parents<Self>, num_cut_points: usize, rng: &mut R) -> Children<Self>
    where
        R: Rng + Sized;
}

impl<V> MultiPointCrossover for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    type Dna = V;

    fn crossover<R>(parents: &Parents<Self>, num_cut_points: usize, rng: &mut R) -> Children<Self>
    where
        R: Rng + Sized,
    {
        let genome_length = parents[0].len();
        let num_parents = parents.len();
        // breed one child for each partner in parents
        let mut offspring: Vec<Vec<V>> = Vec::with_capacity(num_parents);
        while num_parents > offspring.len() {
            let mut genome = Vec::with_capacity(genome_length);
            let mut cutpoints = random_n_cut_points(rng, num_cut_points, genome_length);
            cutpoints.push(genome_length);
            let mut start = 0;
            let mut end = cutpoints.remove(0);
            let mut p_index = num_parents;
            loop {
                loop {
                    let index = rng.gen_range(0..num_parents);
                    if index != p_index {
                        p_index = index;
                        break;
                    }
                }
                let partner = &parents[p_index];
                for partner in partner.iter().take(end).skip(start) {
                    genome.push(partner.clone())
                }
                if cutpoints.is_empty() {
                    break;
                }
                start = end;
                end = cutpoints.remove(0);
            }
            offspring.push(genome);
        }
        offspring
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShuffleCrossBreeder {}

impl ShuffleCrossBreeder {
    pub fn new() -> Self {
        ShuffleCrossBreeder {}
    }
}

impl GeneticOperator for ShuffleCrossBreeder {
    fn name() -> String {
        String::from("Shuffle-Cross-Breeder")
    }
}

impl<G> CrossoverOp<G> for ShuffleCrossBreeder
where
    G: Genotype + ShuffleCrossover,
{
    fn crossover<R>(&self, parents: &Parents<G>, rng: &mut R) -> Children<G>
    where
        R: Rng + Sized,
    {
        ShuffleCrossover::crossover(&parents, rng)
    }
}

pub trait ShuffleCrossover: Genotype {
    type Dna;

    fn crossover<R>(parents: &Parents<Self>, rng: &mut R) -> Children<Self>
    where
        R: Rng + Sized;
}

impl<V> ShuffleCrossover for Vec<V>
where
    V: Clone + Debug + PartialEq + Send + Sync,
{
    type Dna = V;

    fn crossover<R>(parents: &Parents<Self>, rng: &mut R) -> Children<Self>
    where
        R: Rng + Sized,
    {
        let num_parents = parents.len();
        let genome_length = parents[0].len();
        // breed one child for each partner in parents
        let mut offspring: Vec<Vec<V>> = Vec::with_capacity(num_parents);
        let mut p1_index = 0;
        let mut p2_index = 1;
        let single_point_cross_breeder = SinglePointCrossBreeder::new();
        while p1_index < num_parents {
            let mut indices = (0..genome_length).collect::<Vec<usize>>();
            indices.shuffle(rng);
            let shuffled_parent1 = indices.iter().map(|&i| parents[p1_index][i].clone()).collect::<Vec<V>>();
            let shuffled_parent2 = indices.iter().map(|&i| parents[p2_index][i].clone()).collect::<Vec<V>>();

            let parents = vec![shuffled_parent1, shuffled_parent2];
            let childs = single_point_cross_breeder.crossover(&parents, rng);

            let mut child1 = parents[p1_index].clone();
            let mut child2 = parents[p2_index].clone();

            for (idx, &index) in indices.iter().enumerate() {
                child1[index] = childs[0][idx].clone();
                child2[index] = childs[1][idx].clone();
            };

            p1_index += 1;
            p2_index += 1;
            offspring.push(child1);
            offspring.push(child2);
        }
        offspring
    }
}
