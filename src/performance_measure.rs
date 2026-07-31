//! A module that measures the performance of a program. //!

use std::{
    collections::HashMap,
    io::Write,
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

struct Measurements {
    name: String,
    samples: Vec<Duration>,
    max_samples: usize,
    now: Instant,
}

static MEASUREMENTS: OnceLock<Mutex<Measurer>> = OnceLock::new();
fn get_measurer(n: Option<usize>) -> &'static Mutex<Measurer> {
    MEASUREMENTS.get_or_init(|| Mutex::new(Measurer::new(n)))
}

/// A struct that measures the performance of a program.
struct Measurer {
    measurements: HashMap<String, Measurements>,
    max_samples: usize,
}

impl Measurer {
    /// Creates a new Measurer with a default number of samples of 1000.
    pub fn new(num_samples: Option<usize>) -> Self {
        let ns = num_samples.unwrap_or(1000);
        let samples_vec = Vec::with_capacity(ns);
        let measurements: Measurements = Measurements {
            name: "default".to_owned(),
            samples: samples_vec,
            max_samples: ns,
            now: Instant::now(),
        };
        let mut measurement_map: HashMap<String, Measurements> = HashMap::new();
        measurement_map.insert("default".to_owned(), measurements);
        Measurer {
            measurements: measurement_map,
            max_samples: ns,
        }
    }
}

/// Initializes the measurer with some Capacity. Its not required to call this but then the capacity
/// will be locked to 1000
pub fn init(n: usize) {
    get_measurer(Some(n));
}

/// Adds new measurement
pub fn add_measurement(name: &str) {
    let mut measurer = get_measurer(None).lock().unwrap();
    let samples_vec = Vec::with_capacity(measurer.max_samples);
    let measurements: Measurements = Measurements {
        name: name.to_owned(),
        samples: samples_vec,
        max_samples: measurer.max_samples,
        now: Instant::now(),
    };
    measurer.measurements.insert(name.to_owned(), measurements);
}

/// Starts to measure, use stop_measure to stop measuring.
pub fn start_measure() {
    let mut measurer = get_measurer(None).lock().unwrap();
    measurer.measurements.get_mut("default").unwrap().now = Instant::now();
}

/// Starts to measure, use stop_measure to stop measuring.
pub fn start_measure_named(measurement: &str) {
    let mut measurer = get_measurer(None).lock().unwrap();
    let max_samples = measurer.max_samples;
    // possible error due to overwriting
    if measurer.measurements.contains_key(measurement) {
        measurer.measurements.get_mut(measurement).unwrap().now = Instant::now();
    } else {
        measurer.measurements.insert(
            measurement.to_owned(),
            Measurements {
                name: measurement.to_owned(),
                samples: Vec::with_capacity(max_samples),
                max_samples,
                now: Instant::now(),
            },
        );
    }
}

/// Stops measuring and replaces the oldest sample with the new one.
pub fn stop_measure_replace_old() {
    let mut measurer = get_measurer(None).lock().unwrap();
    let elapsed = measurer
        .measurements
        .get_mut("default")
        .unwrap()
        .now
        .elapsed();
    let max_samples = measurer.max_samples;
    let samples = &mut measurer.measurements.get_mut("default").unwrap().samples;
    if samples.len() < max_samples {
        samples.push(elapsed);
    } else {
        samples.remove(0);
        samples.push(elapsed);
    }
}

/// Stops measuring and replaces the oldest sample with the new one.
pub fn stop_measure_replace_old_named(name: &str) {
    let mut measurer = get_measurer(None).lock().unwrap();
    let elapsed = measurer.measurements.get_mut(name).unwrap().now.elapsed();
    let max_samples = measurer.max_samples;
    let samples = &mut measurer.measurements.get_mut(name).unwrap().samples;
    if samples.len() < max_samples {
        samples.push(elapsed);
    } else {
        samples.remove(0);
        samples.push(elapsed);
    }
}

/// Stops measuring and adds the new sample to the list. Does not replace the oldest sample.
pub fn stop_measure() {
    let mut measurer = get_measurer(None).lock().unwrap();
    let elapsed = measurer.measurements.get("default").unwrap().now.elapsed();
    let max_samples = measurer.max_samples;
    let samples = &mut measurer.measurements.get_mut("default").unwrap().samples;
    if samples.len() < max_samples {
        samples.push(elapsed);
    }
}

/// Stops measuring and adds the new sample to the list. Does not replace the oldest sample.
pub fn stop_measure_named(name: &str) {
    let mut measurer = get_measurer(None).lock().unwrap();
    let elapsed = measurer.measurements.get(name).unwrap().now.elapsed();
    let max_samples = measurer.max_samples;
    let samples = &mut measurer.measurements.get_mut(name).unwrap().samples;
    if samples.len() < max_samples {
        samples.push(elapsed);
    }
}

/// Returns the average of all the samples.
pub fn get_average() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    let sum: Duration = samples.iter().cloned().sum();
    sum / samples.len() as u32
}

/// Returns the average of all the samples.
pub fn get_average_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    let sum: Duration = samples.iter().sum();
    sum / samples.len() as u32
}

/// Returns the minimum of all the samples.
pub fn get_min() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    samples
        .iter()
        .cloned()
        .min()
        .unwrap_or(Duration::from_mins(0))
}

/// Returns the minimum of all the samples.
pub fn get_min_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    samples
        .iter()
        .cloned()
        .min()
        .unwrap_or(Duration::from_mins(0))
}

/// Returns the maximum of all the samples.
pub fn get_max() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    samples
        .iter()
        .cloned()
        .max()
        .unwrap_or(Duration::from_mins(0))
}

/// Returns the maximum of all the samples.
pub fn get_max_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    samples
        .iter()
        .cloned()
        .max()
        .unwrap_or(Duration::from_mins(0))
}

/// Returns the median of all the samples.
pub fn get_median() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let mut samples = measurer
        .measurements
        .get("default")
        .unwrap()
        .samples
        .clone();
    samples.sort();
    if samples.len().is_multiple_of(2) {
        (samples[samples.len() / 2] + samples[samples.len() / 2 - 1]) / 2
    } else {
        samples[samples.len() / 2]
    }
}

/// Returns the median of all the samples.
pub fn get_median_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let mut samples = measurer.measurements.get(name).unwrap().samples.clone();
    samples.sort();
    if samples.len().is_multiple_of(2) {
        (samples[samples.len() / 2] + samples[samples.len() / 2 - 1]) / 2
    } else {
        samples[samples.len() / 2]
    }
}

/// Returns the mode of all the samples.
pub fn get_mode() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    let mut map = std::collections::HashMap::new();
    samples.iter().for_each(|x| {
        let count = map.entry(*x).or_insert(0);
        *count += 1;
    });
    let mut max = 0;
    let mut mode = Duration::new(0, 0);
    for (k, v) in map {
        if v > max {
            max = v;
            mode = k;
        }
    }
    mode
}

/// Returns the mode of all the samples.
pub fn get_mode_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    let mut map = std::collections::HashMap::new();
    samples.iter().for_each(|x| {
        let count = map.entry(*x).or_insert(0);
        *count += 1;
    });
    let mut max = 0;
    let mut mode = Duration::new(0, 0);
    for (k, v) in map {
        if v > max {
            max = v;
            mode = k;
        }
    }
    mode
}

/// Returns the standard deviation of all the samples.
pub fn get_std_dev() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    let mean = samples.iter().cloned().sum::<Duration>() / samples.len() as u32;
    let mean_secs = mean.as_secs_f64();

    let sum_squares: f64 = samples
        .iter()
        .map(|x| {
            let diff = x.as_secs_f64() - mean_secs;
            diff * diff
        })
        .sum();
    Duration::from_secs_f64((sum_squares / samples.len() as f64).sqrt())
}

/// Returns the standard deviation of all the samples.
pub fn get_std_dev_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    let mean = samples.iter().cloned().sum::<Duration>() / samples.len() as u32;
    let mean_secs = mean.as_secs_f64();

    let sum_sqaures: f64 = samples
        .iter()
        .map(|x| {
            let diff = x.as_secs_f64() - mean_secs;
            diff * diff
        })
        .sum();
    Duration::from_secs_f64((sum_sqaures / samples.len() as f64).sqrt())
}

/// Returns the variance of all the samples.
pub fn get_variance() -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    let mean = samples.iter().cloned().sum::<Duration>() / samples.len() as u32;
    let mean_secs = mean.as_secs_f64();

    let sum: f64 = samples
        .iter()
        .map(|x| {
            let diff = x.as_secs_f64() - mean_secs;
            diff * diff
        })
        .sum();
    Duration::from_secs_f64(sum / samples.len() as f64)
}

/// Returns the variance of all the samples.
pub fn get_variance_named(name: &str) -> Duration {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    let mean = samples.iter().cloned().sum::<Duration>() / samples.len() as u32;
    let mean_secs = mean.as_secs_f64();

    let sum: f64 = samples
        .iter()
        .map(|x| {
            let diff = x.as_secs_f64() - mean_secs;
            diff * diff
        })
        .sum();
    Duration::from_secs_f64(sum / samples.len() as f64)
}

/// Returns the samples.
pub fn get_samples() -> Vec<Duration> {
    let measurer = get_measurer(None).lock().unwrap();
    measurer
        .measurements
        .get("default")
        .unwrap()
        .samples
        .clone()
}

/// Returns the samples.
pub fn get_samples_named(name: &str) -> Vec<Duration> {
    let measurer = get_measurer(None).lock().unwrap();
    measurer.measurements.get(name).unwrap().samples.clone()
}

/// measures the performance of given closure and returns the average time it took to execute. You can still get the rest of the values via their respective functions.
pub fn measure_closure<F>(mut f: F) -> Duration
where
    F: FnMut(),
{
    let mut measurer = get_measurer(None).lock().unwrap();
    for _ in 0..measurer.max_samples {
        measurer.measurements.get_mut("default").unwrap().now = Instant::now();
        f();
        let elapsed = measurer.measurements.get("default").unwrap().now.elapsed();
        let max_samples = measurer.max_samples;
        let samples = &mut measurer.measurements.get_mut("default").unwrap().samples;
        if samples.len() < max_samples {
            samples.push(elapsed);
        }
    }
    let samples = &measurer.measurements.get("default").unwrap().samples;
    let sum: Duration = samples.iter().cloned().sum();
    sum / samples.len() as u32
}

/// measures the performance of given closure and returns the average time it took to execute. You can still get the rest of the values via their respective functions.
pub fn measure_closure_named<F>(mut f: F, name: &str) -> Duration
where
    F: FnMut(),
{
    let mut measurer = get_measurer(None).lock().unwrap();
    for _ in 0..measurer.max_samples {
        let max_samples = measurer.max_samples;
        // possible error due to overwriting
        if measurer.measurements.contains_key(name) {
            measurer.measurements.get_mut(name).unwrap().now = Instant::now();
        } else {
            measurer.measurements.insert(
                name.to_owned(),
                Measurements {
                    name: name.to_owned(),
                    samples: Vec::with_capacity(max_samples),
                    max_samples,
                    now: Instant::now(),
                },
            );
        }
        f();
        let elapsed = measurer.measurements.get(name).unwrap().now.elapsed();
        let max_samples = measurer.max_samples;
        let samples = &mut measurer.measurements.get_mut(name).unwrap().samples;
        if samples.len() < max_samples {
            samples.push(elapsed);
        }
    }
    let samples = &measurer.measurements.get(name).unwrap().samples;
    let sum: Duration = samples.iter().sum();
    sum / samples.len() as u32
}

/// Saves the samples to a file.
pub fn save_samples(path: &str) -> std::io::Result<()> {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
    let mut file = std::fs::File::create(path)?;
    for sample in samples {
        file.write_all(format!("{}\n", sample.as_secs_f64()).as_bytes())?;
    }
    Ok(())
}

/// Saves the samples to a file.
pub fn save_samples_named(path: &str, name: &str) -> std::io::Result<()> {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
    let mut file = std::fs::File::create(path)?;
    for sample in samples {
        file.write_all(format!("{}\n", sample.as_secs_f64()).as_bytes())?;
    }
    Ok(())
}

/// Saves the samples to a file.
pub fn save_samples_all(path: &str) -> std::io::Result<()> {
    let measurer = get_measurer(None).lock().unwrap();
    let mut file = std::fs::File::create(path)?;
    measurer.measurements.iter().for_each(|v| {
        file.write_all(v.0.clone().as_bytes()).unwrap();
        let samples = v.1.samples.clone();
        for sample in samples {
            file.write_all(format!("{}\n", sample.as_secs_f64()).as_bytes())
                .unwrap();
        }
    });
    Ok(())
}

pub fn reset_measurement() {
    let mut measurer = get_measurer(None).lock().unwrap();
    measurer.measurements.remove("default");
}

pub fn reset_measurement_named(name: &str) {
    let mut measurer = get_measurer(None).lock().unwrap();
    measurer.measurements.remove(name);
}

/// This function plots the times
#[cfg(feature = "plot")]
pub fn plot() {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get("default").unwrap().samples;
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
#[cfg(feature = "plot")]
pub fn plot_named(name: &str) {
    let measurer = get_measurer(None).lock().unwrap();
    let samples = &measurer.measurements.get(name).unwrap().samples;
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
