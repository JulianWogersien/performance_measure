//! A module that measures the performance of a program. //!

#![warn(missing_docs)]

use std::{time::{Duration, Instant}, io::Write, collections::HashMap};

#[allow(dead_code)]
struct Measurements {
    name: String,
    samples: Vec<Duration>,
    max_samples: usize,
    now: Instant,
}

/// A struct that measures the performance of a program.
pub struct Measurer {
    measurements: HashMap<String, Measurements>,
    max_samples: usize,
}

impl Measurer {
    /// Creates a new Measurer with a default number of samples of 1000.
    pub fn new(num_samples: Option<usize>) -> Self {
        let ns = match num_samples {
            Some(s) => s,
            None => 1000,
        };
        let samples_vec = Vec::with_capacity(ns);
        let measurements: Measurements = Measurements { name: "default".to_owned(), samples: samples_vec, max_samples: ns, now: Instant::now() };
        let mut measurement_map: HashMap<String, Measurements> = HashMap::new();
        measurement_map.insert("default".to_owned(), measurements);
        return Measurer { measurements: measurement_map, max_samples: ns};
    }

    /// Adds new measurement
    pub fn add_measurement(&mut self, name: &str) {
        let samples_vec = Vec::with_capacity(self.max_samples);
        let measurements: Measurements = Measurements { name: name.to_owned(), samples: samples_vec, max_samples: self.max_samples, now: Instant::now() };
        self.measurements.insert(name.to_owned(), measurements);
    }

    /// Starts to measure, use stop_measure to stop measuring.
    pub fn start_measure(&mut self) {
        self.measurements.get_mut("default").unwrap().now = Instant::now();
    }

    /// Starts to measure, use stop_measure to stop measuring.
    pub fn start_measure_named(&mut self, measurement: &str) {
        // possible error due to overwriting
        if self.measurements.contains_key(measurement) {
            self.measurements.get_mut(measurement).unwrap().now = Instant::now();
        } else {
            self.measurements.insert(measurement.to_owned(), Measurements { name: measurement.to_owned(), samples: Vec::with_capacity(self.max_samples), max_samples: self.max_samples, now: Instant::now() });
        }
    }

    /// Stops measuring and replaces the oldest sample with the new one.
    pub fn stop_measure_replace_old(&mut self) {
        let elapsed = self.measurements.get_mut("default").unwrap().now.elapsed();
        let samples = &mut self.measurements.get_mut("default").unwrap().samples;
        if samples.len() < self.max_samples {
            samples.push(elapsed);
        } else {
            samples.remove(0);
            samples.push(elapsed);
        }
    }

    /// Stops measuring and replaces the oldest sample with the new one.
    pub fn stop_measure_replace_old_named(&mut self, name: &str) {
        let elapsed = self.measurements.get_mut(name).unwrap().now.elapsed();
        let samples = &mut self.measurements.get_mut(name).unwrap().samples;
        if samples.len() < self.max_samples {
            samples.push(elapsed);
        } else {
            samples.remove(0);
            samples.push(elapsed);
        }
    }

    /// Stops measuring and adds the new sample to the list. Does not replace the oldest sample.
    pub fn stop_measure(&mut self) {
        let elapsed = self.measurements.get("default").unwrap().now.elapsed();
        let samples = &mut self.measurements.get_mut("default").unwrap().samples;
        if samples.len() < self.max_samples {
            samples.push(elapsed);
        }
    }

    /// Stops measuring and adds the new sample to the list. Does not replace the oldest sample.
    pub fn stop_measure_named(&mut self, name: &str) {
        let elapsed = self.measurements.get(name).unwrap().now.elapsed();
        let samples = &mut self.measurements.get_mut(name).unwrap().samples;
        if samples.len() < self.max_samples {
            samples.push(elapsed);
        }
    }

    /// updates the max number of samples to keep.
    pub fn update_max_samples(&mut self, new_max: usize) {
        self.max_samples = new_max;
    }

    /// Returns the average of all the samples.
    pub fn get_average(&self) -> Duration {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut sum = Duration::new(0, 0);
        samples.iter().for_each(|x| sum += *x);
        return sum / samples.len() as u32;
    }

    /// Returns the average of all the samples.
    pub fn get_average_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut sum = Duration::new(0, 0);
        samples.iter().for_each(|x| sum += *x);
        return sum / samples.len() as u32;
    }

    /// Returns the minimum of all the samples.
    pub fn get_min(&self) -> Duration {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut min = samples[0].clone();
        samples.iter().for_each(|x| {
            if *x < min {
                min = *x;
            }
        });
        return min;
    }

    /// Returns the minimum of all the samples.
    pub fn get_min_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut min = samples[0].clone();
        samples.iter().for_each(|x| {
            if *x < min {
                min = *x;
            }
        });
        return min;
    }

    /// Returns the maximum of all the samples.
    pub fn get_max(&self) -> Duration {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut max = Duration::new(0, 0);
        samples.iter().for_each(|x| {
            if *x > max {
                max = *x;
            }
        });
        return max;
    }

    /// Returns the maximum of all the samples.
    pub fn get_max_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut max = Duration::new(0, 0);
        samples.iter().for_each(|x| {
            if *x > max {
                max = *x;
            }
        });
        return max;
    }

    /// Returns the median of all the samples.
    pub fn get_median(&self) -> Duration {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut sorted = samples.clone();
        sorted.sort();
        if sorted.len() % 2 == 0 {
            return (sorted[sorted.len() / 2] + sorted[sorted.len() / 2 - 1]) / 2;
        } else {
            return sorted[sorted.len() / 2];
        }
    }

    /// Returns the median of all the samples.
    pub fn get_median_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut sorted = samples.clone();
        sorted.sort();
        if sorted.len() % 2 == 0 {
            return (sorted[sorted.len() / 2] + sorted[sorted.len() / 2 - 1]) / 2;
        } else {
            return sorted[sorted.len() / 2];
        }
    }

    /// Returns the mode of all the samples.
    pub fn get_mode(&self) -> Duration {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut map = std::collections::HashMap::new();
        samples.iter().for_each(|x| {
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

    /// Returns the mode of all the samples.
    pub fn get_mode_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut map = std::collections::HashMap::new();
        samples.iter().for_each(|x| {
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
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut sum = Duration::new(0, 0);
        let mean = self.get_average();
        samples.iter().for_each(|x| {
            let diff = *x - mean;
            sum += Duration::from_secs(diff.as_secs() * diff.as_secs());
        });
        return Duration::from_secs_f64((sum / samples.len() as u32).as_secs_f64().sqrt());
    }

    /// Returns the standard deviation of all the samples.
    pub fn get_std_dev_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut sum = Duration::new(0, 0);
        let mean = self.get_average();
        samples.iter().for_each(|x| {
            let diff = *x - mean;
            sum += Duration::from_secs(diff.as_secs() * diff.as_secs());
        });
        return Duration::from_secs_f64((sum / samples.len() as u32).as_secs_f64().sqrt());
    }

    /// Returns the variance of all the samples.
    pub fn get_variance(&self) -> Duration {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut sum = Duration::new(0, 0);
        let mean = self.get_average();
        samples.iter().for_each(|x| {
            let diff = *x - mean;
            sum += Duration::from_secs(diff.as_secs() * diff.as_secs());
        });
        return sum / samples.len() as u32;
    }

    /// Returns the variance of all the samples.
    pub fn get_variance_named(&self, name: &str) -> Duration {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut sum = Duration::new(0, 0);
        let mean = self.get_average();
        samples.iter().for_each(|x| {
            let diff = *x - mean;
            sum += Duration::from_secs(diff.as_secs() * diff.as_secs());
        });
        return sum / samples.len() as u32;
    }

    /// Returns the samples.
    pub fn get_samples(&self) -> &Vec<Duration> {
        return &self.measurements.get("default").unwrap().samples;
    }

    /// Returns the samples.
    pub fn get_samples_named(&self, name: &str) -> &Vec<Duration> {
        return &self.measurements.get(name).unwrap().samples;
    }

    /// measures the performance of given closure and returns the average time it took to execute. You can still get the rest of the values via their respective functions.
    pub fn measure_closure<F>(&mut self, mut f: F) -> Duration 
    where
        F: FnMut(),
    {
        for _ in 0..self.max_samples {
            self.start_measure();
            f();
            self.stop_measure();
        }
        return self.get_average();
    }

    /// measures the performance of given closure and returns the average time it took to execute. You can still get the rest of the values via their respective functions.
    pub fn measure_closure_named<F>(&mut self, mut f: F, name: &str) -> Duration 
    where
        F: FnMut(),
    {
        for _ in 0..self.max_samples {
            self.start_measure_named(name);
            f();
            self.stop_measure_named(name);
        }
        return self.get_average_named(name);
    }

    /// Saves the samples to a file.
    pub fn save_samples(&self, path: &str) -> std::io::Result<()> {
        let samples = &self.measurements.get("default").unwrap().samples;
        let mut file = std::fs::File::create(path)?;
        for sample in samples {
            file.write_all(format!("{}\n", sample.as_secs_f64()).as_bytes())?;
        }
        Ok(())
    }

    /// Saves the samples to a file.
    pub fn save_samples_named(&self, path: &str, name: &str) -> std::io::Result<()> {
        let samples = &self.measurements.get(name).unwrap().samples;
        let mut file = std::fs::File::create(path)?;
        for sample in samples {
            file.write_all(format!("{}\n", sample.as_secs_f64()).as_bytes())?;
        }
        Ok(())
    }

    /// Saves the samples to a file.
    pub fn save_samples_all(&self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.measurements.iter().for_each(|v| {
            file.write_all(v.0.clone().as_bytes()).unwrap();
            let samples = v.1.samples.clone();
            for sample in samples {
                file.write_all(format!("{}\n", sample.as_secs_f64()).as_bytes()).unwrap();
            }
        });
        Ok(())
    }

    /// This function plots the times
    #[cfg(feature="plot")]
    pub fn plot(&self) {
        let samples = &self.measurements.get("default").unwrap().samples;
        use graplot::Plot;
        let xvalues: Vec<f64> = (0..samples.len()).map(|v| v as f64).collect();
        let yvales: Vec<f64> = samples.iter().map(|v: &Duration| v.as_secs_f64()).collect();
        let mut plot = Plot::new((xvalues, yvales));
        plot.set_color(0.0, 255.0, 0.0);
        plot.set_title("default");
        plot.set_xlabel("measurements");
        plot.set_ylabel("time in secs");
        plot.show();
    }

    /// This function plots the times
    #[cfg(feature="plot")]
    pub fn plot_named(&self, name: &str) {
        let samples = &self.measurements.get(name).unwrap().samples;
        use graplot::Plot;
        let xvalues: Vec<f64> = (0..samples.len()).map(|v| v as f64).collect();
        let yvales: Vec<f64> = samples.iter().map(|v: &Duration| v.as_secs_f64()).collect();
        let mut plot = Plot::new((xvalues, yvales));
        plot.set_color(0.0, 255.0, 0.0);
        plot.set_title(name);
        plot.set_xlabel("measurements");
        plot.set_ylabel("time in secs");
        plot.show();
    }
}
