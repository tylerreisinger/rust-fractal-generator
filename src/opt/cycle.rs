use num::complex::Complex;

#[derive(Debug, Clone)]
pub struct CycleDetector {
    backlog: Vec<Complex<f64>>,
    write_ptr: usize,
}

impl CycleDetector {
    pub fn new(backlog_len: usize) -> Self {
        let mut vec = Vec::new();
        vec.resize(backlog_len, Complex::new(0.0, 0.0));
        CycleDetector{backlog: vec, write_ptr: 0}
    }

    pub fn check_pt(&mut self, pt: Complex<f64>) -> bool {
        let mut ret = false;

        for prev_pt in self.backlog.iter() {
            if pt == *prev_pt {
                ret = true;
            }
        }

        self.backlog[self.write_ptr] = pt;
        self.write_ptr += 1;
        if self.write_ptr == self.backlog.len() {
            self.write_ptr -= self.backlog.len();
        }

        ret
    }

    pub fn backlog_len(&self) -> usize {
        self.backlog.len()
    }
}
