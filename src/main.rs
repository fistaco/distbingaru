mod entity;
use entity::genetic_algorithm::GeneticAlgorithm;
use entity::crossover_type::CrossoverType;

fn main() {
    let ga: GeneticAlgorithm<u8> = GeneticAlgorithm::<u8>::new(8, 8, 1.0, 1.0, CrossoverType::OnePoint);
    ga.run();
}
