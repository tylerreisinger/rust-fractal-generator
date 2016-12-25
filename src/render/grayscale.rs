
use grid;
use image;
use fractal::{FractalOrbit};
use render::{RenderError, FractalRenderer, RenderResult};
use super::map::{OrbitMapper, MappedCellIntensity};

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

        let mapped_result = self.mapper.map(
            grid.cells_wide(), grid.cells_high(), intensities);
        
        let pixels: Vec<_> = mapped_result.values.iter()
            .map(|item| {
                match *item {
                    MappedCellIntensity::BoundedValue => 0,
                    MappedCellIntensity::EscapedValue(val) => {
                        let pixel_val = val * (u8::max_value() as f64);
                        pixel_val.floor() as u8
                    }
                }
            })
            .collect();

        let buf = image::ImageBuffer::<image::Luma<u8>, _>
               ::from_raw(mapped_result.width as u32, mapped_result.height as u32,
                      pixels).unwrap();

        Ok(image::ImageLuma8(buf))
    }
}

