extern crate image;
extern crate num;
extern crate num_complex;

pub mod grid;
pub mod mandelbrot;
pub mod fractal;

use std::fs;
use std::path;

use num_complex::{Complex};
use fractal::{Fractal};

fn main() {
    let grid = grid::Grid::new(-1.5, 1.0, 1.0, -1.0, 250, 250);
    let mandel = mandelbrot::Mandelbrot::new(250);

    let values = run_fractal(&grid, &mandel);

    let img_buf = build_image(&grid, &values);
    save_image(&img_buf).unwrap();
}

fn run_fractal<T: Fractal>(grid: &grid::Grid, fractal: &T) -> Vec<i32> {
    let num_cells = grid.num_cells();
    let mut values = Vec::with_capacity(num_cells);
    values.resize(num_cells, 0);

    for (i, (x, y)) in grid.iter().enumerate() {
        let iters = fractal.test(Complex::new(x, y));
        values[i] = iters;
    }

    values
}

fn build_image(grid: &grid::Grid, intensities: &Vec<i32>) -> 
        image::DynamicImage {
    let mut pixels = Vec::with_capacity(grid.num_cells());
    pixels.resize(grid.num_cells(), 0 as u8);

    for (i, val) in intensities.iter().enumerate() {
        pixels[i] = *val as u8;
    }

    let buf = image::ImageBuffer::<image::Luma<u8>, _>
        ::from_raw(grid.cells_wide(), grid.cells_high(), pixels);

    image::ImageLuma8(buf.unwrap())
}

fn save_image(img: &image::DynamicImage) -> image::ImageResult<()> {
    let mut file = fs::File::create(&path::Path::new("test.png")).unwrap();
    img.save(&mut file, image::PNG)
}
