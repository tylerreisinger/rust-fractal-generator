
use grid;
use image;
use fractal::{FractalOrbit};
use render::{RenderError, FractalRenderer, RenderResult};
use super::map::OrbitMapper;

pub struct GrayscaleFractalRenderer<T> {
    mapper: T,
}

impl<T: OrbitMapper> GrayscaleFractalRenderer<T> {
    pub fn new(mapper: T) -> Self {
        GrayscaleFractalRenderer{mapper: mapper}
    }
}

impl<T: OrbitMapper> FractalRenderer for GrayscaleFractalRenderer<T> {
    fn render(&self, grid: &grid::Grid, intensities: &[FractalOrbit]) 
            -> RenderResult<image::DynamicImage> {

        if grid.num_cells() != intensities.len() {
            return Err(
                RenderError::OtherError(
                    "Grid and intensities dimensions don't match".to_string()));
        }

        let pixels = self.mapper
            .map(intensities).iter()
            .map(|val| val * (u8::max_value() as f64))
            .map(|val| val.floor() as u8)
            .collect();

        let buf = image::ImageBuffer::<image::Luma<u8>, _>
                   ::from_raw(grid.cells_wide(), grid.cells_high(), pixels).unwrap();

        Ok(image::ImageLuma8(buf))
    }
}

