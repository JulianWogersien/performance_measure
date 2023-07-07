use std::time::{Duration, Instant};

pub struct Measurer {
    samples: Vec<Duration>,
    max_samples: usize,
    now: Instant,
}

impl Measurer {
    /// Creates a new Measurer with a default number of samples of 1000.
    pub fn new(num_samples: Option<usize>) -> Self {
        let ns = match num_samples {
            Some(s) => s,
            None => 1000,
        };
        let samples_vec = Vec::with_capacity(ns);
        return Measurer { samples: samples_vec, max_samples: ns, now: Instant::now() };
    }

    /// Starts to measure, use stop_measure to stop measuring.
    pub fn start_measure(&mut self) {
        self.now = Instant::now();
    }

    /// Stops measuring and replaces the oldest sample with the new one.
    pub fn stop_measure_replace_old(&mut self) {
        let elapsed = self.now.elapsed();
        if self.samples.len() < self.max_samples {
            self.samples.push(elapsed);
        } else {
            self.samples.remove(0);
            self.samples.push(elapsed);
        }
    }

    /// Stops measuring and adds the new sample to the list. Does not replace the oldest sample.
    pub fn stop_measure(&mut self) {
        let elapsed = self.now.elapsed();
        if self.samples.len() < self.max_samples {
            self.samples.push(elapsed);
        }
    }

    /// updates the max number of samples to keep.
    pub fn update_max_samples(&mut self, new_max: usize) {
        self.max_samples = new_max;
    }

    /// Returns the average of all the samples.
    pub fn get_average(&self) -> Duration {

        let mut sum = Duration::new(0, 0);
        self.samples.iter().for_each(|x| sum += *x);
        return sum / self.samples.len() as u32;
    }

    /// Returns the minimum of all the samples.
    pub fn get_min(&self) -> Duration {
        
        let mut min = self.samples[0].clone();
        self.samples.iter().for_each(|x| {
            if *x < min {
                min = *x;
            }
        });
        return min;
    }

    /// Returns the maximum of all the samples.
    pub fn get_max(&self) -> Duration {
        let mut max = Duration::new(0, 0);
        self.samples.iter().for_each(|x| {
            if *x > max {
                max = *x;
            }
        });
        return max;
    }

    /// Returns the median of all the samples.
    pub fn get_median(&self) -> Duration {
        let mut sorted = self.samples.clone();
        sorted.sort();
        if sorted.len() % 2 == 0 {
            return (sorted[sorted.len() / 2] + sorted[sorted.len() / 2 - 1]) / 2;
        } else {
            return sorted[sorted.len() / 2];
        }
    }

    /// Returns the mode of all the samples.
    pub fn get_mode(&self) -> Duration {
        let mut map = std::collections::HashMap::new();
        self.samples.iter().for_each(|x| {
            let count = map.entry(x).or_insert(0);
            *count += 1;
        });
        let mut max = 0;
        let mut mode = Duration::new(0, 0);
        for (k, v) in map {
            if v > max {
                max = v;
                mode = *k;
            }
        }
        return mode;
    }

    /// Returns the standard deviation of all the samples.
    pub fn get_std_dev(&self) -> Duration {
        let mut sum = Duration::new(0, 0);
        let mean = self.get_average();
        self.samples.iter().for_each(|x| {
            let diff = *x - mean;
            sum += Duration::from_secs(diff.as_secs() * diff.as_secs());
        });
        return Duration::from_secs_f64((sum / self.samples.len() as u32).as_secs_f64().sqrt());
    }

    /// Returns the variance of all the samples.
    pub fn get_variance(&self) -> Duration {
        let mut sum = Duration::new(0, 0);
        let mean = self.get_average();
        self.samples.iter().for_each(|x| {
            let diff = *x - mean;
            sum += Duration::from_secs(diff.as_secs() * diff.as_secs());
        });
        return sum / self.samples.len() as u32;
    }

    /// Returns the samples.
    pub fn get_samples(&self) -> &Vec<Duration> {
        return &self.samples;
    }

    /// measures the performance of given closure and returns the average time it took to execute. You can still get the rest of the values via their respective functions.
    pub fn measure_closure<F>(&mut self, f: F) -> Duration
        where F: FnOnce() {
        self.start_measure();
        f();
        self.stop_measure();
        return self.get_average();
    }
}
