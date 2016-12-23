use std::fmt;
use std::error::Error;
use image;
use grid;
use fractal::{FractalOrbit};

pub mod grayscale;
pub mod map;
pub use self::grayscale::GrayscaleFractalRenderer;

#[derive(Debug)]
pub enum RenderError {
    ImageError(image::ImageError),
    OtherError(String),
}

type RenderResult<T> = Result<T, RenderError>;

pub trait FractalRenderer {
    fn render(&self, grid: &grid::Grid, intensities: &[FractalOrbit]) 
        -> RenderResult<image::DynamicImage>;
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
