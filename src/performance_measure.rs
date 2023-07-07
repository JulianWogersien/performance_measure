use std::time::{Duration, Instant};

pub struct Measurer {
    samples: Vec<Duration>,
    max_samples: usize,
    now: Instant,
}

impl Measurer {
    pub fn new(num_samples: Option<usize>) -> Self {
        let ns = match num_samples {
            Some(s) => s,
            None => 1000,
        };
        let samples_vec = Vec::with_capacity(ns);
        return Measurer { samples: samples_vec, max_samples: ns, now: Instant::now() };
    }

    pub fn start_measure(&mut self) {
        self.now = Instant::now();
    }

    pub fn stop_measure(&mut self) {
        let elapsed = self.now.elapsed();
        if self.samples.len() < self.max_samples {
            
        }
    }
}
