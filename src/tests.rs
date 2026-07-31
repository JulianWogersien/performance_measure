use super::performance_measure::*;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static DEFAULT_LOCK: Mutex<()> = Mutex::new(());

fn unique_name(tag: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("test::{tag}::{n}")
}

#[test]
fn init_does_not_panic_regardless_of_call_order() {
    init(64);
    init(64);
}

#[test]
fn add_measurement_creates_an_empty_bucket() {
    let name = unique_name("add_measurement_empty");
    add_measurement(&name);
    assert!(get_samples_named(&name).is_empty());
}

#[test]
fn add_measurement_resets_existing_samples() {
    let name = unique_name("add_measurement_reset");
    add_measurement(&name);
    start_measure_named(&name);
    stop_measure_named(&name);
    assert_eq!(get_samples_named(&name).len(), 1);

    add_measurement(&name);
    assert!(get_samples_named(&name).is_empty());
}

#[test]
fn start_measure_named_on_existing_key_does_not_clear_samples() {
    let name = unique_name("start_named_preserves");
    start_measure_named(&name);
    stop_measure_named(&name);
    assert_eq!(get_samples_named(&name).len(), 1);

    start_measure_named(&name);
    stop_measure_named(&name);
    assert_eq!(get_samples_named(&name).len(), 2);
}

#[test]
fn start_stop_named_records_approximately_the_slept_duration() {
    let name = unique_name("timing_basic");
    start_measure_named(&name);
    thread::sleep(Duration::from_millis(15));
    stop_measure_named(&name);

    let samples = get_samples_named(&name);
    assert_eq!(samples.len(), 1);
    assert!(samples[0] >= Duration::from_millis(15));
    assert!(samples[0] < Duration::from_millis(500));
}

#[test]
fn min_median_max_average_are_consistently_ordered() {
    let name = unique_name("stats_ordering");
    for ms in [1u64, 5, 10, 20] {
        start_measure_named(&name);
        thread::sleep(Duration::from_millis(ms));
        stop_measure_named(&name);
    }

    let min = get_min_named(&name);
    let max = get_max_named(&name);
    let median = get_median_named(&name);
    let avg = get_average_named(&name);

    assert!(
        min <= median,
        "min ({min:?}) should be <= median ({median:?})"
    );
    assert!(
        median <= max,
        "median ({median:?}) should be <= max ({max:?})"
    );
    assert!(
        min <= avg && avg <= max,
        "average ({avg:?}) should be within [min, max]"
    );
    assert!(min >= Duration::from_millis(1));
    assert!(max >= Duration::from_millis(20));
}

#[test]
fn min_max_on_empty_named_bucket_default_to_zero() {
    let name = unique_name("empty_min_max");
    add_measurement(&name);
    assert_eq!(get_min_named(&name), Duration::from_secs(0));
    assert_eq!(get_max_named(&name), Duration::from_secs(0));
}

#[test]
fn mode_returns_a_value_that_was_actually_sampled() {
    let name = unique_name("mode_membership");
    for ms in [1u64, 2, 3] {
        start_measure_named(&name);
        thread::sleep(Duration::from_millis(ms));
        stop_measure_named(&name);
    }
    let samples = get_samples_named(&name);
    let mode = get_mode_named(&name);
    assert!(samples.contains(&mode));
}

#[test]
fn std_dev_and_variance_are_exactly_zero_for_a_single_sample() {
    let name = unique_name("std_dev_single_sample");
    start_measure_named(&name);
    thread::sleep(Duration::from_millis(3));
    stop_measure_named(&name);

    assert_eq!(get_std_dev_named(&name), Duration::from_secs(0));
    assert_eq!(get_variance_named(&name), Duration::from_secs(0));
}

#[test]
fn std_dev_is_larger_for_spread_out_samples_than_uniform_ones() {
    let uniform = unique_name("std_dev_uniform");
    for _ in 0..5 {
        start_measure_named(&uniform);
        thread::sleep(Duration::from_millis(5));
        stop_measure_named(&uniform);
    }

    let spread = unique_name("std_dev_spread");
    for ms in [1u64, 1, 1, 1, 150] {
        start_measure_named(&spread);
        thread::sleep(Duration::from_millis(ms));
        stop_measure_named(&spread);
    }

    let uniform_std = get_std_dev_named(&uniform);
    let spread_std = get_std_dev_named(&spread);

    assert!(
        uniform_std < Duration::from_millis(30),
        "near-identical sleeps should have small std dev, got {uniform_std:?}"
    );
    assert!(
        spread_std > Duration::from_millis(30),
        "widely varying sleeps should have large std dev, got {spread_std:?}"
    );
    assert!(spread_std > uniform_std);
}

#[test]
fn variance_is_approximately_std_dev_squared() {
    let name = unique_name("variance_matches_std_dev");
    for ms in [2u64, 4, 6, 40] {
        start_measure_named(&name);
        thread::sleep(Duration::from_millis(ms));
        stop_measure_named(&name);
    }

    let std_dev = get_std_dev_named(&name).as_secs_f64();
    let variance = get_variance_named(&name).as_secs_f64();

    assert!(
        (std_dev * std_dev - variance).abs() < 1e-6,
        "std_dev^2 ({}) should approximately equal variance ({})",
        std_dev * std_dev,
        variance
    );
}

#[test]
fn get_samples_named_is_a_stable_snapshot() {
    let name = unique_name("samples_snapshot");
    for _ in 0..3 {
        start_measure_named(&name);
        stop_measure_named(&name);
    }
    let a = get_samples_named(&name);
    let b = get_samples_named(&name);
    assert_eq!(a.len(), 3);
    assert_eq!(a, b);
}

#[test]
fn stop_measure_replace_old_named_caps_length_and_keeps_replacing() {
    let name = unique_name("cap_replace");
    let mut last_len = 0usize;
    let mut stable_len = None;

    for _ in 0..2000 {
        start_measure_named(&name);
        stop_measure_replace_old_named(&name);
        let len = get_samples_named(&name).len();
        if len == last_len && len > 0 {
            stable_len = Some(len);
            break;
        }
        last_len = len;
    }

    let cap = stable_len.expect("length should plateau once capacity is reached");

    for _ in 0..25 {
        start_measure_named(&name);
        stop_measure_replace_old_named(&name);
        assert_eq!(get_samples_named(&name).len(), cap);
    }
}

#[test]
fn stop_measure_named_stops_growing_once_capacity_is_reached() {
    let name = unique_name("cap_no_replace");
    let mut last_len = 0usize;
    let mut stable_len = None;

    for _ in 0..2000 {
        start_measure_named(&name);
        stop_measure_named(&name);
        let len = get_samples_named(&name).len();
        if len == last_len && len > 0 {
            stable_len = Some(len);
            break;
        }
        last_len = len;
    }

    let cap = stable_len.expect("length should plateau once capacity is reached");

    for _ in 0..25 {
        start_measure_named(&name);
        stop_measure_named(&name);
        assert_eq!(get_samples_named(&name).len(), cap);
    }
}

#[test]
#[should_panic]
fn get_average_named_panics_for_unknown_name() {
    let name = unique_name("never_added_avg");
    let _ = get_average_named(&name);
}

#[test]
#[should_panic]
fn stop_measure_named_panics_for_unknown_name() {
    let name = unique_name("never_added_stop");
    stop_measure_named(&name);
}

#[test]
#[should_panic]
fn get_samples_named_panics_for_unknown_name() {
    let name = unique_name("never_added_samples");
    let _ = get_samples_named(&name);
}

#[test]
#[should_panic]
fn default_bucket_access_after_reset_panics() {
    let _guard = DEFAULT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    add_measurement("default");
    reset_measurement();
    start_measure();
}

#[test]
fn save_samples_named_writes_one_parseable_line_per_sample() {
    let name = unique_name("save_named");
    for _ in 0..4 {
        start_measure_named(&name);
        stop_measure_named(&name);
    }
    let expected_len = get_samples_named(&name).len();

    let path = std::env::temp_dir().join(format!("perf_test_{}.txt", unique_name("file")));
    let path_str = path.to_str().unwrap();

    save_samples_named(path_str, &name).expect("save_samples_named should succeed");

    let contents = std::fs::read_to_string(&path).expect("file should exist and be readable");
    let lines: Vec<&str> = contents.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), expected_len);
    for line in lines {
        line.parse::<f64>()
            .unwrap_or_else(|_| panic!("line '{line}' should parse as f64"));
    }

    let _ = std::fs::remove_file(&path);
}

#[test]
fn save_samples_all_includes_our_named_measurement() {
    let name = unique_name("save_all_marker");
    start_measure_named(&name);
    stop_measure_named(&name);

    let path = std::env::temp_dir().join(format!("perf_test_all_{}.txt", unique_name("file")));
    save_samples_all(path.to_str().unwrap()).expect("save_samples_all should succeed");

    let contents = std::fs::read_to_string(&path).expect("file should exist and be readable");
    assert!(
        contents.contains(&name),
        "output should contain our measurement's name"
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn default_bucket_start_stop_and_stats_are_self_consistent() {
    let _guard = DEFAULT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    add_measurement("default");

    let before_len = get_samples().len();

    start_measure();
    thread::sleep(Duration::from_millis(10));
    stop_measure();

    let samples = get_samples();
    assert_eq!(
        samples.len(),
        before_len + 1,
        "exactly one sample should be appended"
    );

    let min = get_min();
    let max = get_max();
    let avg = get_average();
    assert!(min <= avg && avg <= max);
    assert!(max >= Duration::from_millis(10));
}

#[test]
fn default_bucket_save_samples_matches_current_length() {
    let _guard = DEFAULT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    add_measurement("default");

    start_measure();
    stop_measure();
    let expected_len = get_samples().len();

    let path = std::env::temp_dir().join(format!("perf_test_default_{}.txt", unique_name("file")));
    save_samples(path.to_str().unwrap()).expect("save_samples should succeed");

    let contents = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = contents.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), expected_len);

    let _ = std::fs::remove_file(&path);
}

#[test]
fn measure_closure_named_invokes_closure_once_per_recorded_sample() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let name = unique_name("measure_closure_named");
    let calls = AtomicUsize::new(0);

    let returned_avg = measure_closure_named(
        || {
            calls.fetch_add(1, Ordering::Relaxed);
        },
        &name,
    );

    let samples = get_samples_named(&name);
    assert!(!samples.is_empty());
    assert_eq!(calls.load(Ordering::Relaxed), samples.len());
    assert_eq!(returned_avg, get_average_named(&name));

    reset_measurement_named(&name);
}

#[test]
fn measure_closure_runs_against_the_default_bucket_and_returns_consistent_average() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let _guard = DEFAULT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    add_measurement("default");

    let calls = AtomicUsize::new(0);
    let returned_avg = measure_closure(|| {
        calls.fetch_add(1, Ordering::Relaxed);
    });

    let samples = get_samples();
    assert!(!samples.is_empty());
    assert_eq!(calls.load(Ordering::Relaxed), samples.len());
    assert_eq!(returned_avg, get_average());

    add_measurement("default");
}

#[test]
fn reset_measurement_named_clears_prior_samples() {
    let name = unique_name("reset_named");
    start_measure_named(&name);
    stop_measure_named(&name);
    assert_eq!(get_samples_named(&name).len(), 1);

    reset_measurement_named(&name);

    add_measurement(&name);
    assert!(get_samples_named(&name).is_empty());
}

#[test]
fn reset_measurement_named_on_an_already_missing_name_is_a_harmless_no_op() {
    let name = unique_name("reset_never_added");
    reset_measurement_named(&name);
}

#[test]
fn default_bucket_reset_then_readd_is_empty() {
    let _guard = DEFAULT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    add_measurement("default");
    start_measure();
    stop_measure();
    assert_eq!(get_samples().len(), 1);

    reset_measurement();
    add_measurement("default");

    assert!(get_samples().is_empty());
}
