use std::{thread, time::Duration};

use slow_function_warning::*;

#[debug_slow_function_warning(1000ms)]
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

#[test]
fn small_component() {
    sleep(1000);
}
