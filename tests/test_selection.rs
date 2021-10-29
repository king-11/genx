extern crate genx;

#[cfg(test)]
mod tests {
    use genx::selection::{random::random_selection, rank::rank_selection, roulette_wheel::roulette_wheel_selection, steady_state::steady_state_selection, stochastic_universal::stochastic_universal_selection, tournament::tournament_selection};

    #[test]
    fn test_random_selection() {
      let num_parents:usize = 10;
      let fitness_values = vec![2.4,5.6,2.3,1.2,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

      let result = random_selection(fitness_values.len(), num_parents, Some(42)).iter().map(|&a| fitness_values[a]).collect::<Vec<f32>>();
      assert_eq!(result,[2.4, 9.0, 2.3, 7.7, 0.6, 4.4, 4.8, 8.4, 10.0, 0.2]);
    }

    #[test]
    fn test_rank_selection() {
      let num_parents:usize = 10;
      let fitness_values = vec![2.4,5.6,2.3,1.2,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

      let result = rank_selection(&fitness_values, num_parents, Some(42)).iter().map(|&a| fitness_values[a]).collect::<Vec<f32>>();
      assert_eq!(result,[3.2, 8.4, 4.8, 8.4, 10.0, 9.0, 11.0, 5.6, 11.0, 1.2]);
    }

    #[test]
    fn test_roulette_wheel_selection() {
      let num_parents:usize = 10;
      let fitness_values = vec![2.4,5.6,2.3,1.2,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

      let result = roulette_wheel_selection(&fitness_values, num_parents, Some(42)).iter().map(|&a| fitness_values[a]).collect::<Vec<f32>>();
      assert_eq!(result,[4.4, 7.7, 10.0, 7.7, 11.0, 3.2, 4.5, 9.0, 4.5, 5.6]);
    }

    #[test]
    fn test_steady_state_selection() {
      let num_parents:usize = 10;
      let fitness_values = vec![2.4,5.6,2.3,1.2,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

      let result = steady_state_selection(&fitness_values, num_parents).iter().map(|&a| fitness_values[a]).collect::<Vec<f32>>();
      assert_eq!(result,[11.0, 10.0, 9.4, 9.0, 9.0, 8.4, 7.7, 5.6, 5.6, 4.8]);
    }

    #[test]
    fn test_stochastic_universal_selection() {
      let num_parents:usize = 10;
      let fitness_values = vec![2.4,5.6,2.3,1.2,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

      let result = stochastic_universal_selection(&fitness_values, num_parents, Some(42)).iter().map(|&a| fitness_values[a]).collect::<Vec<f32>>();
      assert_eq!(result,[2.4, 0.6, 5.6, 10.0, 9.0, 7.7, 8.4, 9.4, 9.0, 11.0]);
    }

    #[test]
    fn test_tournament_selection() {
      let num_parents:usize = 10;
      let fitness_values = vec![2.4,5.6,2.3,1.2,0.6,4.4,2.3,5.6,10.0,0.2,9.0,4.8,7.7,8.4,3.2,9.4,9.0,11.0,4.5];

      let result = tournament_selection(&fitness_values, num_parents, 5, Some(42)).iter().map(|&a| fitness_values[a]).collect::<Vec<f32>>();
      assert_eq!(result,[11.0, 10.0, 5.6, 9.0, 8.4, 10.0, 9.4, 11.0, 9.4, 10.0]);
    }
}
