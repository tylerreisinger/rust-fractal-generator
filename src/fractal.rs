use num_complex::{Complex};
use opt::cycle::CycleDetector;

pub type EscapeTimeType = f64;

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum FractalOrbit {
    Escaped(EscapeTimeType),
    Bounded,
}

pub trait Fractal: Clone {
    fn test(&self, c: Complex<f64>) -> FractalOrbit;
}

#[derive(Clone)]
pub struct FractalExecutor<T> {
    fractal_impl: T,
    cycle_detector: CycleDetector,
}

impl<T: Fractal> FractalExecutor<T> {
    pub fn new(fractal_impl: T, cycle_detector: CycleDetector) -> Self {
        FractalExecutor{fractal_impl: fractal_impl, cycle_detector: cycle_detector}
    }
}

impl<T: Fractal> Fractal for FractalExecutor<T> {
    fn test(&self, c: Complex<f64>) -> FractalOrbit {
        self.fractal_impl.test(c)
    }
}
