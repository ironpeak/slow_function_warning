use std::{thread, time::Duration};

use slow_function_warning::*;

#[test]
fn module() {
    #[slow_function_warning(1ms, {*value = module.to_string();})]
    pub fn sleep(millis: u64, value: &mut String) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = "".to_string();
    sleep(2, &mut value);

    assert_eq!(value, "variables");
}

#[test]
fn function() {
    #[slow_function_warning(1ms, {*value = function.to_string();})]
    pub fn sleep(millis: u64, value: &mut String) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = "".to_string();
    sleep(2, &mut value);

    assert_eq!(value, "sleep");
}

#[test]
fn elapsed() {
    #[slow_function_warning(1ms, {*duration = elapsed.clone();})]
    pub fn sleep(millis: u64, duration: &mut Duration) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut duration = Duration::default();
    sleep(2, &mut duration);

    assert_eq!(duration.as_millis(), 2);
}

#[test]
fn elapsed_str() {
    #[slow_function_warning(1ms, {*value = elapsed_str.clone();})]
    pub fn sleep(millis: u64, value: &mut String) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = "".to_string();
    sleep(2, &mut value);

    assert_eq!(value, "2ms");
}

#[test]
fn elapsed_ns() {
    #[slow_function_warning(1ms, {*value = elapsed_ns;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert!(value > 2000000);
}

#[test]
fn elapsed_nanos() {
    #[slow_function_warning(1ms, {*value = elapsed_nanos;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert!(value > 2000000);
}

#[test]
fn elapsed_nanoseconds() {
    #[slow_function_warning(1ms, {*value = elapsed_nanoseconds;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert!(value > 2000000);
}

#[test]
fn elapsed_us() {
    #[slow_function_warning(1ms, {*value = elapsed_us;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert!(value > 2000);
}

#[test]
fn elapsed_micros() {
    #[slow_function_warning(1ms, {*value = elapsed_micros;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert!(value > 2000);
}

#[test]
fn elapsed_microseconds() {
    #[slow_function_warning(1ms, {*value = elapsed_microseconds;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert!(value > 2000);
}

#[test]
fn elapsed_ms() {
    #[slow_function_warning(1ms, {*value = elapsed_ms;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 2);
}

#[test]
fn elapsed_millis() {
    #[slow_function_warning(1ms, {*value = elapsed_millis;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 2);
}

#[test]
fn elapsed_milliseconds() {
    #[slow_function_warning(1ms, {*value = elapsed_milliseconds;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 2);
}

#[test]
fn elapsed_s() {
    #[slow_function_warning(1ms, {*value = elapsed_s;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_secs() {
    #[slow_function_warning(1ms, {*value = elapsed_secs;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_seconds() {
    #[slow_function_warning(1ms, {*value = elapsed_seconds;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_m() {
    #[slow_function_warning(1ms, {*value = elapsed_m;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_min() {
    #[slow_function_warning(1ms, {*value = elapsed_min;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_minutes() {
    #[slow_function_warning(1ms, {*value = elapsed_minutes;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_h() {
    #[slow_function_warning(1ms, {*value = elapsed_h;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_hours() {
    #[slow_function_warning(1ms, {*value = elapsed_hours;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_d() {
    #[slow_function_warning(1ms, {*value = elapsed_d;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn elapsed_days() {
    #[slow_function_warning(1ms, {*value = elapsed_days;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit() {
    #[slow_function_warning(1ms, {*duration = limit.clone();})]
    pub fn sleep(millis: u64, duration: &mut Duration) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut duration = Duration::default();
    sleep(2, &mut duration);

    assert_eq!(duration.as_millis(), 1);
}

#[test]
fn limit_str() {
    #[slow_function_warning(1ms, {*value = limit_str.clone();})]
    pub fn sleep(millis: u64, value: &mut String) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = "".to_string();
    sleep(2, &mut value);

    assert_eq!(value, "1ms");
}

#[test]
fn limit_ns() {
    #[slow_function_warning(1ms, {*value = limit_ns;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1000000);
}

#[test]
fn limit_nanos() {
    #[slow_function_warning(1ms, {*value = limit_nanos;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1000000);
}

#[test]
fn limit_nanoseconds() {
    #[slow_function_warning(1ms, {*value = limit_nanoseconds;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1000000);
}

#[test]
fn limit_us() {
    #[slow_function_warning(1ms, {*value = limit_us;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1000);
}

#[test]
fn limit_micros() {
    #[slow_function_warning(1ms, {*value = limit_micros;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1000);
}

#[test]
fn limit_microseconds() {
    #[slow_function_warning(1ms, {*value = limit_microseconds;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1000);
}

#[test]
fn limit_ms() {
    #[slow_function_warning(1ms, {*value = limit_ms;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1);
}

#[test]
fn limit_millis() {
    #[slow_function_warning(1ms, {*value = limit_millis;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1);
}

#[test]
fn limit_milliseconds() {
    #[slow_function_warning(1ms, {*value = limit_milliseconds;})]
    pub fn sleep(millis: u64, value: &mut u128) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u128::default();
    sleep(2, &mut value);

    assert_eq!(value, 1);
}

#[test]
fn limit_s() {
    #[slow_function_warning(1ms, {*value = limit_s;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_secs() {
    #[slow_function_warning(1ms, {*value = limit_secs;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_seconds() {
    #[slow_function_warning(1ms, {*value = limit_seconds;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_m() {
    #[slow_function_warning(1ms, {*value = limit_m;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_min() {
    #[slow_function_warning(1ms, {*value = limit_min;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_minutes() {
    #[slow_function_warning(1ms, {*value = limit_minutes;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_h() {
    #[slow_function_warning(1ms, {*value = limit_h;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_hours() {
    #[slow_function_warning(1ms, {*value = limit_hours;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_d() {
    #[slow_function_warning(1ms, {*value = limit_d;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}

#[test]
fn limit_days() {
    #[slow_function_warning(1ms, {*value = limit_days;})]
    pub fn sleep(millis: u64, value: &mut u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut value = u64::default();
    sleep(2, &mut value);

    assert_eq!(value, 0);
}
