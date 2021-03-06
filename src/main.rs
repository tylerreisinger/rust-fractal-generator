extern crate image;
extern crate num;
extern crate num_complex;

pub mod grid;
pub mod mandelbrot;
pub mod fractal;
pub mod runner;
pub mod render;
pub mod opt;

use std::fs;
use std::path;

use render::{FractalRenderer};
use runner::FractalRunner;

fn main() {
    const MAX_ITERS: usize = 500;

    let grid = grid::Grid::new(-1.5, 1.0, 1.0, -1.0, 2000, 2000);
    let mandel = mandelbrot::Mandelbrot::new(MAX_ITERS);

    let runner = runner::MultiThreadedRunner::new(mandel, 2);
    let renderer = 
    render::GrayscaleFractalRenderer::new(
        render::map::AntialiasMapper::new(1,
            render::map::LogarithmicMapper::new(MAX_ITERS, 100.0))
    );

    let intensities = runner.run(&grid).unwrap();
    let image = renderer.render(&grid, &intensities).unwrap();

    save_image(&image).unwrap();
}

fn save_image(img: &image::DynamicImage) -> image::ImageResult<()> {
    let mut file = fs::File::create(&path::Path::new("test.png"))?;
    img.save(&mut file, image::PNG)
}
