use num_complex::{Complex};

pub trait Fractal {
    fn test(&self, c: Complex<f64>) -> i32;
}
