use rand::{Rng, rngs::ThreadRng};
use super::genome::Genome;


/// A struct representing a bit string of variable size.
#[derive(Clone)]
pub struct BinaryString {
    bits: Vec<u8>,
    pub fitness: f64
}

impl BinaryString {
    /// Constructs a new `BinaryString` with a bit string initialised with `size` zeros. 
    pub fn new(size: usize) -> Self {
        BinaryString {
            bits: vec![0; size],
            fitness: 0.0
        }
    }

    /// Returns a `BinaryString` with a bit string of length `size` in which the bits are uniformly
    /// randomly set to 0 or 1.
    pub fn random(size: usize, mut rng: ThreadRng) -> Self {
        let random_bits: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=1)).collect();

        BinaryString {
            fitness: random_bits.iter().sum::<u8>() as f64,
            bits: random_bits
        }
        
    }
}

impl Genome for BinaryString {
    /// Computes and returns this `BinaryString`'s fitness value, which is computed as the sum of
    /// its bits that are set to 1.
    fn fitness(&self) -> f64 {
        self.bits.iter().sum::<u8>() as f64
    }

    /// Sets this instance's `fitness` to the given value.
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
}
