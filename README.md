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

You can plot the times using the plot function\
To use plotting you have to enable the "plot" feature

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

    // Retrieve other statistics if needed
    let min_time = measurer.get_min();
    let max_time = measurer.get_max();
    let median_time = measurer.get_median();
    let variance = measurer.get_variance();
    let std_deviation = measurer.get_std_deviation();
    let mode = measurer.get_mode();

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
