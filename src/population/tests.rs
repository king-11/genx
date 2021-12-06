use super::*;

mod population_builder {

    use super::*;
    use crate::random::{get_rng, random_seed};
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn builds_a_population_of_any_number_of_vec_of_bool_genomes(
            size in 0usize..9_999,
        ) {
            let rng = get_rng(random_seed());

            let population: Population<Vec<bool>> = PopulationBuilder::build_population(
                &BinaryEncodedGenomeBuilder::new(42),
                size,
                rng,
            );

            prop_assert_eq!(population.size(), size);
        }
    }
}
