use genx::{
    crossover::uniform_crossover, mutation::inversion_mutation, selection::random_selection,
    selection::stochastic_universal_selection,
};
use rand::{distributions::Uniform, prelude::Distribution};

fn prune_database(
    database: Vec<Vec<(i32, i32)>>,
    cost: &Vec<f32>,
    min_util: f32,
    num_items: i32,
) -> Vec<Vec<(i32, i32)>> {
    let mut utils = vec![0.0; num_items as usize];
    for i in database.iter() {
        let mut val = 0.0;
        for j in i.iter() {
            val += cost[j.0 as usize] * j.1 as f32;
        }
        for j in i.iter() {
            utils[j.0 as usize] += val;
        }
    }
    let mut new_database: Vec<Vec<(i32, i32)>> = Vec::new();
    for i in database.iter() {
        let mut new_transaction: Vec<(i32, i32)> = Vec::new();
        for j in i.iter() {
            if utils[j.0 as usize] >= min_util {
                new_transaction.push(j.clone());
            }
        }
        if new_transaction.len() > 0 {
            new_database.push(new_transaction.clone());
        }
    }
    new_database
}

fn main() {
    let num_items = 5;
    let cost = vec![2.0, 3.0, 5.0, 6.0, 1.0];
    let mut database = vec![
        vec![(0, 2), (1, 6)],
        vec![(2, 1), (4, 1)],
        vec![(3, 2), (0, 3)],
    ];
    let min_util: f32 = 10.0;
    database = prune_database(database.clone(), &cost, min_util, num_items);
    let mut bit_database: Vec<Vec<bool>> = Vec::new();
    for i in database.iter() {
        let mut bit: Vec<bool> = vec![false; num_items as usize];
        for j in i.iter() {
            bit[j.0 as usize] = true;
            println!("{},{}", j.0, j.1);
        }
        bit_database.push(bit.clone());
    }
    let iterations = 300;
    let population_size = 20;
    let mutation_probability: f32 = 0.4;
    let mut population = (0..population_size)
        .into_iter()
        .map(|_| (0..num_items).map(|_| rand::random::<bool>()).collect())
        .collect::<Vec<Vec<bool>>>();

    let fitness_function = |x: &Vec<bool>, cost: &Vec<f32>| {
        let mut res = 0.0;
        for (index, val) in database.iter().enumerate() {
            let mut flag = 1;
            for j in 0..num_items {
                if x[j as usize] == true && bit_database[index][j as usize] == false {
                    flag = 0;
                }
            }
            let mut ut = 0.0;
            if flag == 1 {
                for j in val.iter() {
                    if x[j.0 as usize] {
                        ut += j.1 as f32 * cost[j.0 as usize];
                    }
                }
            }
            res += ut;
        }
        res
    };
    let best_fitness_value = |population: &Vec<Vec<bool>>, cost: &Vec<f32>| {
        population
            .iter()
            .max_by(|&a, &b| {
                fitness_function(a, &cost)
                    .partial_cmp(&fitness_function(b, &cost))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
            .clone()
    };
    let get_fitness_values = |population: &Vec<Vec<bool>>, cost: &Vec<f32>| {
        population
            .iter()
            .map(|x| fitness_function(x, &cost))
            .collect::<Vec<f32>>()
    };

    let mut best_now = best_fitness_value(&population, &cost);
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

        let fitness_values = get_fitness_values(&population, &cost);
        let selected_idx = stochastic_universal_selection(&fitness_values, population_size, None);
        population = selected_idx
            .iter()
            .map(|&a| population[a].clone())
            .collect::<Vec<Vec<bool>>>();
        let best = best_fitness_value(&population, &cost);
        if fitness_function(&best, &cost) > fitness_function(&best_now, &cost) {
            best_now = best;
        }
        println!("Fitness {}", fitness_function(&best_now,&cost));
    }
}
