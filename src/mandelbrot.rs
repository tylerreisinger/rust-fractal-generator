use num_complex::{Complex};

use fractal::{Fractal, FractalOrbit, EscapeTimeType};

#[derive(Clone)]
pub struct Mandelbrot {
    iter_limit: i32,
}

impl Mandelbrot {
    pub fn new(iter_limit: i32) -> Self {
        Mandelbrot{iter_limit: iter_limit}
    }

    pub fn check_carteoid_inclusion(&self, c: &Complex<f64>) -> bool{
        let q = (c.re - 0.25) * (c.re - 0.25) + c.im*c.im;

        if q*(q + c.re - 0.25) < 0.25 * c.im*c.im {
            return true;
        } else if (c.re + 1.0)*(c.re + 1.0) + c.im*c.im < 1.0/16.0 {
            return true;
        }

        false
    }
}

impl Fractal for Mandelbrot {
    fn test(&self, c: Complex<f64>) -> FractalOrbit {
        const MAX_RADIUS_SQR: f64 = 4.0;

        if self.check_carteoid_inclusion(&c) {
            return FractalOrbit::Bounded;
        }

        let mut z = Complex::new(0.0, 0.0);
        let mut i: i32 = 0;
        
        while z.norm_sqr() < MAX_RADIUS_SQR {
            i += 1;
            z = z*z + c;
            if i >= self.iter_limit {
                return FractalOrbit::Bounded;
            }
        }

        FractalOrbit::Escaped(i as EscapeTimeType)
    } 
}
