use fractal::FractalOrbit;

pub trait OrbitMapper {
    fn map(&self, width: usize, height: usize, vals: &[FractalOrbit]) -> MappingResult; 
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MappedCellIntensity {
    EscapedValue(f64),
    BoundedValue,
}

pub struct MappingResult {
    pub values: Vec<MappedCellIntensity>,
    pub width: usize,
    pub height: usize,
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

pub struct AntialiasMapper<T> {
    aa_level: u32,
    mapper: T,
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
    fn map(&self, width: usize, height: usize, vals: &[FractalOrbit]) 
            -> MappingResult {
        let vals = vals.iter()
            .map(|o| self.map(*o))
            .collect();
        MappingResult{values: vals, width: width, height: height}
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
    fn map(&self, width: usize, height: usize, vals: &[FractalOrbit]) 
            -> MappingResult {
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

        MappingResult{values: output, width: width, height: height}
    }
}

impl LogarithmicMapper {
    pub fn new(max_iter: usize, strength: f64) -> Self {
        LogarithmicMapper{max_iter: max_iter, strength: strength}
    }
}

impl OrbitMapper for LogarithmicMapper {
    fn map(&self, width: usize, height: usize, vals: &[FractalOrbit]) 
            -> MappingResult {
        let max = self.max_iter as f64;
        let multiplier = self.strength + 1.0;
        let vals = vals.iter()
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
            .collect::<Vec<_>>();

        MappingResult{values: vals, width: width, height: height}
    }
}

impl<T: OrbitMapper> AntialiasMapper<T> {
    pub fn new(aa_level: u32, mapper: T) -> Self {
        AntialiasMapper{aa_level: aa_level, mapper: mapper}
    }
}

impl<T: OrbitMapper> OrbitMapper for AntialiasMapper<T> {
    fn map(&self, width: usize, height: usize, vals: &[FractalOrbit]) 
            -> MappingResult {
        let reduction_factor = 1 << self.aa_level;
        let flt_reduction_factor = (1 << self.aa_level) as f64;
        let scaled_width = ((width as f64) / flt_reduction_factor).floor() as usize;
        let scaled_height = ((height as f64) / flt_reduction_factor).floor() as usize;

        let mut out = vec![BoundedValue; scaled_width*scaled_height];
        
        let full_img = self.mapper.map(width, height, vals);
        let width = full_img.width;
        let orig_values = full_img.values;

        let denom = flt_reduction_factor*flt_reduction_factor;
        for y in 0..scaled_height {
            let y_start = y*reduction_factor;
            for x in 0..scaled_width {
                let x_start = x*reduction_factor;
                let mut final_value = 0.0;

                for y0 in y_start..y_start+reduction_factor {
                    for x0 in x_start..x_start+reduction_factor {
                        let item = orig_values[x0 + y0*width];
                        final_value += match item {
                            BoundedValue => 1.0,
                            EscapedValue(val) => val,
                        }
                    }
                }

                out[x + y*scaled_width] = 
                    if final_value == 1.0*denom {
                        BoundedValue
                    } else {
                        EscapedValue(
                            final_value / (flt_reduction_factor*flt_reduction_factor))
                    }
            }
        }

        MappingResult{values: out, width: scaled_width, height: scaled_height}
    }
}
