use std::fmt;
use std::error::Error;

use image;

use grid;

#[derive(Debug)]
pub enum RenderError {
    ImageError(image::ImageError),
    OtherError(String),
}

type RenderResult<T> = Result<T, RenderError>;

pub trait FractalRenderer<Input> {
    fn render(&self, grid: &grid::Grid, intensities: &[Input]) 
        -> RenderResult<image::DynamicImage>;
}

pub struct BwFractalRenderer {
    max_iter: i32,
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderError::ImageError(ref e) => e.fmt(f),
            RenderError::OtherError(ref msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl Error for RenderError {
    fn description(&self) -> &str {
        match *self {
            RenderError::ImageError(ref e) => e.description(),
            RenderError::OtherError(ref msg) => &msg,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RenderError::ImageError(ref e) => Some(e),
            RenderError::OtherError(_) => None,
        }
    }
}

impl From<image::ImageError> for RenderError {
    fn from(other: image::ImageError) -> Self {
        RenderError::ImageError(other)
    }
}

impl BwFractalRenderer {
    pub fn new(max_iter: i32) -> Self {
        BwFractalRenderer{max_iter: max_iter}
    }
}

impl FractalRenderer<i32> for BwFractalRenderer {
    fn render(&self, grid: &grid::Grid, intensities: &[i32]) 
            -> RenderResult<image::DynamicImage> {

        if grid.num_cells() != intensities.len() {
            return Err(
                RenderError::OtherError(
                    "Grid and intensities dimensions don't match".to_string()));
        }

        let mut pixels = Vec::with_capacity(grid.num_cells());
        pixels.resize(grid.num_cells(), 0 as u8);

        for (pixel, intensity) in pixels.iter_mut().zip(intensities.iter()) {
            let mut float_intensity = (*intensity as f64) / (self.max_iter as f64);
            float_intensity *= 255.0; 
            *pixel = float_intensity.floor() as u8;
        }

        let buf = image::ImageBuffer::<image::Luma<u8>, _>
                   ::from_raw(grid.cells_wide(), grid.cells_high(), pixels).unwrap();

        Ok(image::ImageLuma8(buf))
    }
}

