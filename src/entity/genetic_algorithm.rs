use super::crossover_type::CrossoverType;
use super::genome::genome::Genome;

/// Represents the state of a genetic algorithm and defines methods to run it.
pub struct GeneticAlgorithm<T: Genome> {
    population: Vec<T>,
    population_size: usize,
    num_generations: usize,
    fitness_values: Vec<f64>,
    optimal_fitness: f64,
    crossover_rate: f32,
    mutation_rate: f32,
    crossover_type: CrossoverType
}

impl<T: Genome> GeneticAlgorithm<T> {
    /// Constructs a new `GeneticAlgorithm` with the given parameters.
    pub fn new(population_size: usize, num_generations: usize, optimal_fitness: f64,
        crossover_rate: f32, mutation_rate: f32, crossover_type: CrossoverType) -> Self {
                GeneticAlgorithm {
                    population: Vec::<T>::with_capacity(population_size),
                    population_size,
                    num_generations,
                    fitness_values: Vec::<f64>::with_capacity(population_size),
                    optimal_fitness,
                    crossover_rate,
                    mutation_rate,
                    crossover_type
                }
               }
    
    /// Runs the genetic algorithm with this GA instance's parameters.
    /// 
    /// # Returns
    /// The genome with the best fitness after the GA has terminated. 
    pub fn run(&mut self) {

        println!("Running genetic algorithm with population size {}", self.population_size);

        for generation in 0..self.num_generations {
            self.evaluate_fitness();

            self.tournament_select();
            self.produce_offspring();
        }
    }

    /// Computes and sets the fitness values of all genomes in the population.
    fn evaluate_fitness(&mut self) {
        self.population.iter_mut().for_each(|genome| { genome.set_fitness(genome.fitness()); });
    }

    fn tournament_select(&self) {

    }

    fn produce_offspring(&self) {

    }
}
