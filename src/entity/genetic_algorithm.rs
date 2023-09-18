use super::crossover_type::CrossoverType;

/// Represents the state of a genetic algorithm and defines methods to run it.
pub struct GeneticAlgorithm<T> {
    population: Vec<T>,
    population_size: usize,
    num_generations: usize,
    optimal_fitness: i32,
    crossover_rate: f32,
    mutation_rate: f32,
    crossover_type: CrossoverType
}

impl<T> GeneticAlgorithm<T> {
    pub fn new(population_size: usize, num_generations: usize, optimal_fitness: i32,
        crossover_rate: f32, mutation_rate: f32, crossover_type: CrossoverType) -> Self {
                GeneticAlgorithm {
                    population: Vec::<T>::new(),
                    population_size,
                    num_generations,
                    optimal_fitness,
                    crossover_rate,
                    mutation_rate,
                    crossover_type
                }
               }
    
    pub fn run(&self) {

        println!("Running genetic algorithm with population size {}", self.population_size);

        for i in 0..self.num_generations {
            self.evaluate_fitness();
            self.tournament_select();
            self.produce_offspring();
        }
    }

    fn evaluate_fitness(&self) {

    }

    fn tournament_select(&self) {

    }

    fn produce_offspring(&self) {

    }
}
