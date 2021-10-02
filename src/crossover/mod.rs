
pub mod single_point;

pub mod multi_point;

pub mod shuffle;

pub mod uniform;


pub use self::single_point::single_point_crossover;

pub use self::multi_point::multi_point_crossover;

pub use self::shuffle::shuffle_crossover;

pub use self::uniform::uniform_crossover;

fn check_length<T>(parent1 : &Vec<T>, parent2 : &Vec<T>) {
  if parent1.len() != parent2.len() {
    panic!("Vectors must be the same length");
  }
}
