use std::{cmp::Ordering, f32::MAX};

use genx::{
    crossover::order_crossover,
    mutation::scramble_mutation,
    selection::{random_selection, stochastic_universal_selection},
};

use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, thread_rng};

fn main() {
    let graph = vec![
        vec![MAX, 5.0, 6.0, 4.56, 9.84, 3.56, 11.25, 9.80],
        vec![5.0, MAX, 3.0, 2.0, 7.0, 1.0, 8.0, 6.0],
        vec![6.0, 3.0, MAX, 2.0, 6.0, 1.0, 7.0, 5.0],
        vec![4.56, 2.0, 2.0, MAX, 3.0, 1.0, 4.0, 3.0],
        vec![9.84, 7.0, 6.0, 3.0, MAX, 2.0, 5.0, 4.0],
        vec![3.56, 1.0, 1.0, 1.0, 2.0, MAX, 3.0, 2.0],
        vec![11.25, 8.0, 7.0, 4.0, 5.0, 3.0, MAX, 2.0],
        vec![9.80, 6.0, 5.0, 3.0, 4.0, 2.0, 2.0, MAX],
    ];
    let n = graph.len();
    let population_size = 15;
    let iterations = 100;
    let mutation_probability = 0.5;
    let between = Uniform::from(0.0..1.0);
    let mut prng = thread_rng();

    let fitness_function = |a: &Vec<usize>| {
        let mut sum = 0.0;
        let mut prev_idx = a[0];
        for (i, &idx) in a.iter().enumerate() {
            if i == 0 {
                continue;
            }
            sum += graph[prev_idx][idx];
            prev_idx = idx;
        }

        sum
    };

    let mut population = (0..population_size)
        .map(|_| {
            let mut a = (0..n).collect::<Vec<usize>>();
            a.shuffle(&mut prng);
            a
        })
        .collect::<Vec<Vec<usize>>>();

    let get_fitness_values = |population: &Vec<Vec<usize>>| {
        population
            .iter()
            .map(|a| fitness_function(a))
            .collect::<Vec<f32>>()
    };

    let mut best_now = population[0].clone();

    for _ in 0..iterations {
        let idxs = random_selection(population_size, 2, None);
        let (mut child1, mut child2) =
            order_crossover(&population[idxs[0]], &population[idxs[1]], None);

        if between.sample(&mut prng) < mutation_probability {
            scramble_mutation(&mut child1, None);
        }
        if between.sample(&mut prng) < mutation_probability {
            scramble_mutation(&mut child2, None);
        }

        population.push(child1);
        population.push(child2);
        let fitness_values = get_fitness_values(&population);
        let selected_idx = stochastic_universal_selection(&fitness_values, population_size, None);
        population = selected_idx
            .iter()
            .map(|&a| population[a].clone())
            .collect::<Vec<Vec<usize>>>();

        let best = population
            .iter()
            .min_by(|a, b| {
                fitness_function(a)
                    .partial_cmp(&fitness_function(b))
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap();

        if fitness_function(best) < fitness_function(&best_now) {
            best_now = best.clone();
        }
    }
    print!("{:?} {}", best_now, fitness_function(&best_now));
}
