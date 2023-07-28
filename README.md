# Performance Measure for Rust

[![Build test](https://github.com/coolian1337/performance_measure/actions/workflows/rust.yml/badge.svg)](https://github.com/coolian1337/performance_measure/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/performance_measure.svg)](https://crates.io/crates/performance_measure)

The Performance Measure library in Rust allows you to measure the performance of
your code and obtain essential statistics like average, minimum, maximum,
median, variance, standard deviation, and mode. This library is particularly
useful for optimizing and analyzing the efficiency of your Rust code.

## Getting Started

To start measuring performance, follow these steps:

1. Create a new Measurer instance with `Measurer::new(Option::None)`. The
   optional parameter allows you to control the number of samples the Measurer
   will keep. The default is 1000 samples.

2. You have two options for measuring performance:
   - **Using Closures**: Call `measure_closure` on the Measurer variable,
     passing in the closure you want to measure. This function will return the
     average time it took to execute the closure.
   - **Manual Measurement**: Call `start_measure` to start the timer, then
     execute the code you want to measure. After executing the code, call either
     `stop_measure` to add a sample (if the maximum sample limit hasn't been
     reached) or `stop_measure_replace_old` to replace old samples once the
     maximum limit is reached. This method is more suitable for measuring
     performance inside loops.

## Named Functions for Multiple Measurements

New named functions have been added to allow you to have multiple measurements within one Measurer instance. These named functions allow you to keep track of different measurements separately. Here's how you can use them:

1. **Start Named Measurement**: Call `start_measure_named("measurement_name")` to start a new named measurement. Replace `"measurement_name"` with a descriptive name for your measurement.

2. **Stop Named Measurement**: Call `stop_measure_named("measurement_name")` to stop the named measurement specified by the given name. This will add a sample to the named measurement.

3. **Stop Named Measurement and Replace Old Samples**: Call `stop_measure_replace_old_named("measurement_name")` to stop the named measurement and, if the maximum sample limit hasn't been reached, add a new sample. Once the maximum limit is reached, this function will replace old samples in the named measurement.

4. **Retrieve Named Measurement Statistics**: After stopping a named measurement, you can retrieve various statistics for that specific measurement, just like with the default measurement. Use functions like `get_min_named`, `get_max_named`, `get_median_named`, `get_variance_named`, `get_std_deviation_named`, `get_mode_named`, and `get_samples_named` to access the statistics of the named measurement.

## Default Measurement

The non-named functions now work on the measurement called "default" by default. If you don't explicitly specify a measurement name while using the `start_measure`, `stop_measure`, or `stop_measure_replace_old` functions, they will operate on the "default" measurement.

## Available Statistics

You can retrieve various statistics after measuring performance, including:

- Average time
- Minimum time
- Maximum time
- Median time
- Variance
- Standard deviation
- Mode
- Raw samples

To access these statistics, call the corresponding functions provided by the
Measurer instance.

## Plotting

You can plot the times using the plot function. To use plotting, you have to enable the "plot" feature.

## Saving Samples

If you wish to save the measured samples to a file, you can use the
`save_samples` function provided by the Measurer instance.

## Example Usage

Here's an example of how to use the Performance Measure library:

```rust
use performance_measure::Measurer;

fn main() {
    // Create a Measurer with the default number of samples (1000)
    let mut measurer = Measurer::new(None);

    // Using closure measurement
    let average_time = measurer.measure_closure(|| {
        // Code to be measured goes here
        // For example, a time-consuming function or a loop
    });

    println!("Average time: {:.2} ms", average_time);

    // Manual measurement using start_measure and stop_measure
    measurer.start_measure();
    // Code to be measured goes here
    // For example, a time-consuming function inside a loop
    measurer.stop_measure();

    // Start a named measurement
    measurer.start_measure_named("named_measurement");

    // Code for the named measurement goes here
    // For example, another time-consuming function inside a loop

    // Stop the named measurement
    measurer.stop_measure_named("named_measurement");

    // Retrieve statistics for the named measurement
    let named_min_time = measurer.get_min_named("named_measurement");
    let named_max_time = measurer.get_max_named("named_measurement");
    let named_median_time = measurer.get_median_named("named_measurement");
    let named_variance = measurer.get_variance_named("named_measurement");
    let named_std_deviation = measurer.get_std_deviation_named("named_measurement");
    let named_mode = measurer.get_mode_named("named_measurement");

    // Plot the times
    measurer.plot();

    // Save samples to a file
    measurer.save_samples("performance_samples.txt").unwrap();
}
```
  
## Contributing

If you find any issues or have suggestions for improvements, feel free to
contribute to this project by creating pull requests or opening issues.

We hope the Performance Measure library proves to be a valuable tool in
optimizing and analyzing the performance of your Rust code. Happy coding!