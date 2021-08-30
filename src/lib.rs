use std::cmp::{Ordering, max};
use rand::{thread_rng, seq::SliceRandom, Rng};

pub fn rank_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
    let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
    for (i, &value) in fitness_values.iter().enumerate() {
        fitness_values_with_index.push((value, i));
    };
    fitness_values_with_index.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let mut ranks:Vec<i32> = vec![1];
    let mut sum_of_fitness = 1;
    for (i, _) in fitness_values_with_index.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if fitness_values_with_index[i].0 != fitness_values_with_index[i-1].0 {
            ranks.push(ranks[i-1] + 1);
        }
        else {
            ranks.push(ranks[i-1]);
        }
        sum_of_fitness += ranks[i];
    };
    let normalized_probabilities = ranks.iter().map(|&a| (a as f32)/(sum_of_fitness as f32)).collect::<Vec<f32>>();

    let mut selected_indices:Vec<usize> = Vec::new();
    for _ in 0..num_parents {
        let val = rand::thread_rng().gen();
        let mut cummulative_probability = 0f32;
        for (i, &(_, idx)) in fitness_values_with_index.iter().enumerate() {
            cummulative_probability += normalized_probabilities[i];
            if cummulative_probability >= val {
                selected_indices.push(idx);
                break;
            }
        };
    };
    selected_indices
}

pub fn roulette_wheel_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
    let mut sum_of_fitness = 1f32;
    for &val in fitness_values.iter() {
        sum_of_fitness += val;
    };
    let normalized_probabilities = fitness_values.iter().map(|&a| a/sum_of_fitness).collect::<Vec<f32>>();

    let mut selected_indices:Vec<usize> = Vec::new();
    for _ in 0..num_parents {
        let val = rand::thread_rng().gen();
        let mut cummulative_probability = 0f32;
        for (index, _) in fitness_values.iter().enumerate() {
            cummulative_probability += normalized_probabilities[index];
            if cummulative_probability >= val {
                selected_indices.push(index);
                break;
            }
        };
    };
    selected_indices
}

pub fn random_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
    let population_size = fitness_values.len();
    let mut selected_indices:Vec<usize> = Vec::new();
    for _ in 0..num_parents {
        let val = thread_rng().gen_range(0..population_size);
        selected_indices.push(val);
    };
    selected_indices
}

pub fn steady_state_selection(fitness_values: &Vec<f32>, num_parents: usize) -> Vec<usize> {
    let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
    let population_size = fitness_values.len();
    for (i, &value) in fitness_values.iter().enumerate() {
        fitness_values_with_index.push((value, i));
    };
    fitness_values_with_index.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
    let selected_indices = fitness_values_with_index.iter().map(|&a| a.1).collect::<Vec<usize>>();
    selected_indices[0..max(num_parents, population_size)].to_vec()
}

pub fn tournament_selection(fitness_values: &Vec<f32>, num_parents: usize, tournament_size: usize) -> Vec<usize> {
    let mut fitness_values_with_index : Vec<(f32, usize)> = Vec::new();
    for (i, &value) in fitness_values.iter().enumerate() {
        fitness_values_with_index.push((value, i));
    };

    let mut selected_indices:Vec<usize> = Vec::new();
    for _ in 0..num_parents {
        let mut rng = thread_rng();
        let mut current_tournament = fitness_values_with_index.choose_multiple(&mut rng, tournament_size).cloned().collect::<Vec<(f32, usize)>>();
        current_tournament.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        selected_indices.push(current_tournament[tournament_size-1].1);
    };

    selected_indices
}
