pub trait Genome {
    fn fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
}
