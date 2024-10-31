use slow_function_warning::*;
use std::{thread, time::Duration};

fn write_to_file(path: &str, message: &str) {
    std::fs::write(path, message).unwrap();
}

fn read_from_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[test]
fn warn() {
    #[debug_slow_function_warning(10ms, write_to_file("./debug_slow_function_warning_warn.txt", &format!("{module}::{function}")))]
    pub fn sleep(millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }
    sleep(10);

    assert_eq!(
        read_from_file("./debug_slow_function_warning_warn.txt"),
        "debug_slow_function_warning::sleep"
    );
}

#[test]
fn no_warn() {
    #[debug_slow_function_warning(10ms, write_to_file("./debug_slow_function_warning_no_warn.txt", &format!("{module}::{function}")))]
    pub fn sleep(millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }
    sleep(1);

    assert!(!file_exists("./debug_slow_function_warning_no_warn.txt"));
}
