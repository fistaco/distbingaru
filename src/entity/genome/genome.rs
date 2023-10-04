pub trait Genome {
    fn fitness(&mut self) -> f64;
}
