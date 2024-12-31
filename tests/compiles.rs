use slow_function_warning::*;

#[test]
fn default_compiles() {
    #[slow_function_warning]
    pub fn sleep(millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    sleep(1);
}

#[test]
fn default_warning_compiles() {
    #[slow_function_warning(10ms)]
    pub fn sleep(millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    sleep(1);
}
