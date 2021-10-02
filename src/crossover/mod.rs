use std::collections::HashSet;

pub mod single_point;

pub mod multi_point;

pub mod shuffle;

pub mod uniform;

pub mod partially_mapped;

pub mod order;

pub mod cycle;

pub use self::single_point::single_point_crossover;

pub use self::multi_point::multi_point_crossover;

pub use self::shuffle::shuffle_crossover;

pub use self::uniform::uniform_crossover;

pub use self::partially_mapped::partially_mapped_crossover;

pub use self::order::davis_order_crossover;

pub use self::cycle::cycle_crossover;

fn check_continuous(vec: &Vec<usize>) -> bool {
  let n = vec.len();
  let mut set:HashSet<usize> = HashSet::new();
  for x in vec.iter() {
    set.insert(*x);
    if *x >= n {
      return false;
    }
  }

  set.len() == vec.len()
}

fn check_length<T>(parent1 : &Vec<T>, parent2 : &Vec<T>) {
  if parent1.len() != parent2.len() {
    panic!("Vectors must be the same length");
  }
}
