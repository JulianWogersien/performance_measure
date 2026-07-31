# performance_measure

[![Build test](https://github.com/coolian1337/performance_measure/actions/workflows/rust.yml/badge.svg)](https://github.com/coolian1337/performance_measure/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/performance_measure.svg)](https://crates.io/crates/performance_measure)

A tiny helper for timing bits of your Rust code and pulling stats out of the
results: average, min, max, median, mode, variance, standard deviation The
raw samples, whatever you need.
Under the hood it keeps a rolling buffer of samples (1000 by default) so you
can drop this into a hot loop without it growing forever.

## Quick start

```rust
use performance_measure::*;

fn main() {
    start_measure();
    do_something_expensive();
    stop_measure();

    println!("average: {:?}", get_average());
    println!("min: {:?}, max: {:?}", get_min(), get_max());
}
```

Setup is done Automatically when calling any of the functions unless you want to set your own maximum capacity.

If you'd rather just time a closure:

```rust
let avg = measure_closure(|| {
    do_something_expensive();
});
```

`measure_closure` runs your closure to fill the sample buffer (so up to the maximum capacity), and hands back the average. You can still pull `min`,
`max`, `median`, etc. afterward — it fills the same sample buffer as
`start_measure`/`stop_measure` would.

## Naming measurements

`start_measure` / `stop_measure` and friends all operate on one shared
bucket called `"default"`. If you want to track more than one thing at once for example, parsing vs rendering use the `_named` versions and give each its own name:

```rust
start_measure_named("parsing");
parse_input();
stop_measure_named("parsing");

start_measure_named("rendering");
render_frame();
stop_measure_named("rendering");

println!("parse avg: {:?}", get_average_named("parsing"));
println!("render avg: {:?}", get_average_named("rendering"));
```

You don't need to explicitly create a named measurement first
`start_measure_named` will create it the first time it sees a new name. If
you want an empty bucket before you start timing (or want to
wipe one clean), `add_measurement("parsing")` does that.

## Two ways to stop a measurement

- `stop_measure()` / `stop_measure_named(name)` — adds a new sample, but
  once you hit the capacity limit it just stops recording.
- `stop_measure_replace_old()` / `stop_measure_replace_old_named(name)` —
  once full, this replaces the oldest sample with the new one. Use this
  for stuff you're measuring in a loop, so your stats stay a rolling
  window of recent runs instead of freezing at whatever happened first.

## Stats

Once you've got samples in a bucket, these are all available (each has a
`_named(name)` version too):

- `get_average()`
- `get_min()` / `get_max()`
- `get_median()`
- `get_mode()`
- `get_variance()`
- `get_std_dev()`
- `get_samples()` the raw `Vec<Duration>`, if you want to do your own math

## Capacity

By default each measurement keeps up to 1000 samples. If you want something
different, call `init(n)` once, before doing anything else:

```rust
init(200); // keep the last 200 samples per measurement
```

One thing to watch out for: the capacity is set globally, the first time
anything touches the measurer, whether that's an explicit `init()` call or
just the first `start_measure()` you happen to call. Whichever one runs
first wins, and it can't be changed afterward. So if you care about a
specific capacity, call `init()` before any measuring happen.

## Resetting

If you want to clear out a measurement and start fresh, `reset_measurement()`
(or `reset_measurement_named(name)`) will drop it entirely.

## Saving samples to disk

```rust
save_samples("default_samples.txt")?;
save_samples_named("parsing_samples.txt", "parsing")?;

// or dump everything you've recorded so far, all measurements at once
save_samples_all("all_samples.txt")?;
```

Each line in the file is one sample, in seconds, as a float.

## Plotting

There's a `plot()` / `plot_named(name)` function that pops up a quick plot
of your samples, but it's behind the `plot` feature flag since it pulls in
a plotting dependency you might not want:

```toml
[dependencies]
performance_measure = { version = "...", features = ["plot"] }
```

## Contributing

Found a bug or have an idea for a feature? Issues and PRs are welcome.
