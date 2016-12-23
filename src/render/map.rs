use fractal::FractalOrbit;

pub trait OrbitMapper {
    fn map(&self, vals: &[FractalOrbit]) -> Vec<f64>; 
}

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

    fn map(&self, val: FractalOrbit) -> f64 {
        match val {
            FractalOrbit::Bounded => 1.0,
            FractalOrbit::Escaped(time) => time / (self.max_iter as f64)
        }
    }
}

impl OrbitMapper for LinearMapper {
    fn map(&self, vals: &[FractalOrbit]) -> Vec<f64> {
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
    fn map(&self, vals: &[FractalOrbit]) -> Vec<f64> {
        let mut histogram = vec![0; self.max_iter+1];

        for orbit in vals.iter() {
            match *orbit {
                FractalOrbit::Bounded => histogram[self.max_iter] += 1,
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
                    FractalOrbit::Bounded => self.max_iter as f64,
                    FractalOrbit::Escaped(val) => val,
                }
            })
            .map(|item| {
                let trunc = item.floor();
                let bin_val = mapping[trunc as usize];
                bin_val
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
    fn map(&self, vals: &[FractalOrbit]) -> Vec<f64> {
        let max = self.max_iter as f64;
        let multiplier = self.strength + 1.0;
        vals.iter()
            .map(|item| {
                match *item {
                    FractalOrbit::Bounded => 1.0,
                    FractalOrbit::Escaped(val) => val / max,
                }
            })
            .map(|val| {
                f64::log10(val * multiplier + 1.0) / f64::log10(multiplier + 1.0)
            })
            .collect::<Vec<f64>>()
    }
}
