use std::fmt::{Debug, Display};

use genx::{
    crossover::uniform_crossover,
    mutation::inversion_mutation,
    selection::{random_selection, stochastic_universal_selection},
};

use rand::{distributions::Uniform, prelude::Distribution};

struct Item {
    weight: u32,
    value: u32,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Item {{ weight: {}, value: {} }}",
            self.weight, self.value
        )
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Item {{ weight: {}, value: {} }}",
            self.weight, self.value
        )
    }
}

fn main() {
    let knapsack_threshold = 80;
    let knapsack_items = vec![
        Item {
            weight: 9,
            value: 150,
        },
        Item {
            weight: 13,
            value: 35,
        },
        Item {
            weight: 15,
            value: 10,
        },
        Item {
            weight: 50,
            value: 60,
        },
        Item {
            weight: 15,
            value: 60,
        },
        Item {
            weight: 68,
            value: 45,
        },
        Item {
            weight: 27,
            value: 60,
        },
        Item {
            weight: 39,
            value: 40,
        },
        Item {
            weight: 23,
            value: 30,
        },
        Item {
            weight: 52,
            value: 10,
        },
        Item {
            weight: 11,
            value: 70,
        },
        Item {
            weight: 32,
            value: 30,
        },
        Item {
            weight: 24,
            value: 15,
        },
    ];
    let knapsack_size = knapsack_items.len();

    let iterations = 300;
    let population_size = 20;
    let mutation_probability = 0.4;
    let mut population = (0..population_size)
        .into_iter()
        .map(|_| (0..knapsack_size).map(|_| rand::random::<bool>()).collect())
        .collect::<Vec<Vec<bool>>>();

    let fitness_function = |individual: &Vec<bool>| -> f32 {
        let mut total_weight = 0;
        let mut total_value = 0;
        for (item, &is_included) in knapsack_items.iter().zip(individual.iter()) {
            if is_included {
                total_weight += item.weight;
                total_value += item.value;
            }
        }
        if total_weight > knapsack_threshold {
            total_value = 0;
        }
        1.0 + total_value as f32
    };

    let get_fitness_values = |population: &Vec<Vec<bool>>| {
        population
            .iter()
            .map(|x| fitness_function(x))
            .collect::<Vec<f32>>()
    };

    let best_fitness_value = |population: &Vec<Vec<bool>>| {
        population
            .iter()
            .max_by(|&a, &b| {
                fitness_function(a)
                    .partial_cmp(&fitness_function(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
            .clone()
    };

    let mut best_now = best_fitness_value(&population);
    let between = Uniform::from(0.0..1.0);
    let mut prng = rand::thread_rng();
    for _ in 0..iterations {
        let idxs = random_selection(population_size, 2, None);
        let (mut child1, mut child2) =
            uniform_crossover(&population[idxs[0]], &population[idxs[1]], 0.5, None);

        if between.sample(&mut prng) < mutation_probability {
            inversion_mutation(&mut child1, None);
        }
        if between.sample(&mut prng) < mutation_probability {
            inversion_mutation(&mut child2, None);
        }
        population.push(child1);
        population.push(child2);

        let fitness_values = get_fitness_values(&population);
        let selected_idx = stochastic_universal_selection(&fitness_values, population_size, None);
        population = selected_idx
            .iter()
            .map(|&a| population[a].clone())
            .collect::<Vec<Vec<bool>>>();
        let best = best_fitness_value(&population);
        if fitness_function(&best) > fitness_function(&best_now) {
            best_now = best;
        }
    }

    println!("{}", fitness_function(&best_now));
    println!("{:?}", best_now.iter().zip(knapsack_items).filter(|predicate| *predicate.0).map(|x| x.1).collect::<Vec<Item>>());
}
