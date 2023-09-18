use super::crossover_type::CrossoverType;

pub struct GeneticAlgorithm<'a, T: 'a> {
    population: &'a [T],
    population_size: u32,
    optimal_fitness: i32,
    crossover_rate: f32,
    mutation_rate: f32,
    crossover_type: CrossoverType
}

impl<'a, T: 'a> GeneticAlgorithm<'a, T> {
    // TODO
}
