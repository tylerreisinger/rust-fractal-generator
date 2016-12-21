
use num_complex::{Complex};
use fractal::{Fractal};
use grid;

pub struct FractalRunner<T: Fractal> {
    fractal: T,
}

impl<T: Fractal> FractalRunner<T> {
    pub fn new(fractal: T) -> FractalRunner<T> {
        FractalRunner::<T>{fractal: fractal}
    }

    pub fn run(&self, grid: &grid::Grid) -> Vec<i32> {
        let mut values = Vec::with_capacity(grid.num_cells());
        values.resize(grid.num_cells(), 0);

        for (i, (x, y)) in grid.iter().enumerate() {
            let iters = self.fractal.test(Complex::new(x, y));
            values[i] = iters;
        }

        values
    }

}

impl<'a, T: Fractal> FractalRunner<T> {
    pub fn fractal(&'a self) -> &'a T {
        &self.fractal
    }
}
