use fractal::FractalOrbit;

pub trait OrbitMapper {
    fn map(&self, vals: &[FractalOrbit]) -> Vec<MappedCellIntensity>; 
}

pub enum MappedCellIntensity {
    EscapedValue(f64),
    BoundedValue,
}

use self::MappedCellIntensity::{EscapedValue, BoundedValue};

pub struct LinearMapper {
    max_iter: usize,
}

pub struct HistogramLinearMapper {
    max_iter: usize,
}

pub struct LogarithmicMapper {
    max_iter: usize,
    strength: f64,
}

impl LinearMapper {
    pub fn new(max_iter: usize) -> Self {
        LinearMapper{max_iter: max_iter}
    }

    fn map(&self, val: FractalOrbit) -> MappedCellIntensity {
        match val {
            FractalOrbit::Bounded => BoundedValue,
            FractalOrbit::Escaped(time) => 
                EscapedValue(time / (self.max_iter as f64))
        }
    }
}

impl OrbitMapper for LinearMapper {
    fn map(&self, vals: &[FractalOrbit]) -> Vec<MappedCellIntensity> {
        vals.iter()
            .map(|o| self.map(*o))
            .collect()
    }
}

impl HistogramLinearMapper {
    pub fn new(max_iter: usize) -> Self {
        let mut hist = Vec::with_capacity(max_iter);
        hist.resize(max_iter, 0);

        HistogramLinearMapper{max_iter: max_iter}
    }
}

impl OrbitMapper for HistogramLinearMapper {
    fn map(&self, vals: &[FractalOrbit]) -> Vec<MappedCellIntensity> {
        let mut histogram = vec![0; self.max_iter+1];

        for orbit in vals.iter() {
            match *orbit {
                FractalOrbit::Bounded => {},
                FractalOrbit::Escaped(val) => histogram[val.floor() as usize] += 1,
            }
        }

        let total = vals.len() as f64;
        let mapping: Vec<_> = histogram.iter()
            .scan(0.0, |state, item| {
                *state += *item as f64;
                Some(*state / total)
            })
            .collect();

        let output: Vec<_> = vals.iter()
            .map(|item| {
                match *item {
                    FractalOrbit::Bounded => BoundedValue,
                    FractalOrbit::Escaped(val) => {
                        let trunc = val.floor();
                        let bin_val = mapping[trunc as usize];
                        EscapedValue(bin_val)
                    }
                }
            })
            .collect();

        output
    }
}

impl LogarithmicMapper {
    pub fn new(max_iter: usize, strength: f64) -> Self {
        LogarithmicMapper{max_iter: max_iter, strength: strength}
    }
}

impl OrbitMapper for LogarithmicMapper {
    fn map(&self, vals: &[FractalOrbit]) -> Vec<MappedCellIntensity> {
        let max = self.max_iter as f64;
        let multiplier = self.strength + 1.0;
        vals.iter()
            .map(|item| {
                match *item {
                    FractalOrbit::Bounded => BoundedValue,
                    FractalOrbit::Escaped(val) => {
                        let scaled_val = val / max;
                        let mapped_val = f64::log10(scaled_val * multiplier + 1.0) 
                            / f64::log10(multiplier + 1.0);
                        EscapedValue(mapped_val)
                    }
                }
            })
            .collect::<Vec<_>>()
    }
}
