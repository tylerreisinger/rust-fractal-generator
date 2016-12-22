use num_complex::{Complex};

use fractal::{Fractal};

#[derive(Clone)]
pub struct Mandelbrot {
    iter_limit: i32,
}

impl Mandelbrot {
    pub fn new(iter_limit: i32) -> Self {
        Mandelbrot{iter_limit: iter_limit}
    }
}

impl Fractal for Mandelbrot {
    fn test(&self, c: Complex<f64>) -> i32 {
        const MAX_RADIUS_SQR: f64 = 4.0;
        let mut z = Complex::new(0.0, 0.0);
        let mut i: i32 = 0;
        
        while z.norm_sqr() < MAX_RADIUS_SQR {
            i += 1;
            z = z*z + c;
            if i >= self.iter_limit {
                break;
            }
        }

        i 
    }
}
