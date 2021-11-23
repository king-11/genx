use std::{cmp::Ordering, io::Read};

use genx::{
    crossover::partially_mapped_crossover,
    mutation::scramble_mutation,
    scaling::linear_scaling,
    selection::{random_selection, stochastic_universal_selection},
};

use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, thread_rng};

fn read_data() -> Vec<Vec<f32>> {
    let mut data = vec![];
    // FRI26 is a set of 26 cities, from TSPLIB. The minimal tour has length 937.
    // 1 25 24 23 26 22 21 17 18 20 19 16 11 12 13 15 14 10 9 8 7 5 6 4 3 2 1
    // best tour
    let mut file = std::fs::File::open("examples/data/tsp.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    for line in buffer.lines() {
        let temp_line = line.split_whitespace();
        let mut current_row = vec![];
        for x in temp_line {
            current_row.push(x.parse::<f32>().unwrap());
        }
        data.push(current_row);
    }
    data
}

fn main() {
    let graph = read_data();
    let n = graph.len();
    let population_size = 200;
    let iterations = 5000;
    let mutation_probability = 0.5;
    let between = Uniform::from(0.0..1.0);
    let mut prng = thread_rng();

    let distance = |a: &Vec<usize>| {
        let mut sum = 0.0;
        let mut prev_idx = a[0];
        for &idx in a.iter() {
            sum += graph[prev_idx][idx];
            prev_idx = idx;
        }

        sum
    };

    let fitness_function = |a: &Vec<usize>| {
        let sum = distance(a);

        1.0 / (sum.sqrt().sqrt() + 1.0)
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
            partially_mapped_crossover(&population[idxs[0]], &population[idxs[1]], None);

        if between.sample(&mut prng) < mutation_probability {
            scramble_mutation(&mut child1, None);
        }
        if between.sample(&mut prng) < mutation_probability {
            scramble_mutation(&mut child2, None);
        }

        population.push(child1);
        population.push(child2);
        let mut fitness_values = get_fitness_values(&population);
        linear_scaling(&mut fitness_values, 1.2);
        let selected_idx = stochastic_universal_selection(&fitness_values, population_size, None);
        population = selected_idx
            .iter()
            .map(|&a| population[a].clone())
            .collect::<Vec<Vec<usize>>>();

        let best = population
            .iter()
            .max_by(|a, b| {
                fitness_function(a)
                    .partial_cmp(&fitness_function(b))
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap();

        if fitness_function(best) > fitness_function(&best_now) {
            best_now = best.clone();
        }
    }
    //
    print!("{:?} {}", best_now, distance(&best_now));
}
