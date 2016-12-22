use num_complex::{Complex};

pub type EscapeTimeType = f64;

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum FractalOrbit {
    Escaped(EscapeTimeType),
    Bounded,
}

pub trait Fractal: Clone {
    fn test(&self, c: Complex<f64>) -> FractalOrbit;
}
