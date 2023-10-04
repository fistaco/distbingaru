use rand::Rng;

mod entity;
use entity::genetic_algorithm::GeneticAlgorithm;
use entity::crossover_type::CrossoverType;
use entity::genome::binary_string::BinaryString;


fn main() {
    let mut rng = rand::thread_rng();
    let binary_string: BinaryString = BinaryString::random(8, rng);
    println!("{:?}", binary_string.fitness);

    let pop_size: usize = 8;
    let mut ga: GeneticAlgorithm<BinaryString> = GeneticAlgorithm::<BinaryString>::new(
        pop_size,
        10,
        pop_size as f64,
        1.0,
        1.0,
        CrossoverType::OnePoint
    );
    ga.run();
}
