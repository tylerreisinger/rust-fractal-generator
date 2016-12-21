extern crate image;
extern crate num;
extern crate num_complex;

pub mod grid;
pub mod mandelbrot;
pub mod fractal;
pub mod runner;
pub mod render;

use std::fs;
use std::path;

use render::{FractalRenderer};

fn main() {
    let grid = grid::Grid::new(-1.5, 1.0, 1.0, -1.0, 250, 250);
    let mandel = mandelbrot::Mandelbrot::new(250);

    let runner = runner::FractalRunner::new(mandel);
    let renderer = render::BwFractalRenderer::new(250);

    let intensities = runner.run(&grid);
    let image = renderer.render(&grid, &intensities).unwrap();
    save_image(&image).unwrap();
}

fn save_image(img: &image::DynamicImage) -> image::ImageResult<()> {
    let mut file = fs::File::create(&path::Path::new("test.png")).unwrap();
    img.save(&mut file, image::PNG)
}
