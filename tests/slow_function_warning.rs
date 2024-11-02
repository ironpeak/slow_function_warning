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
fn default_compiles() {
    #[slow_function_warning]
    pub fn sleep(millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    sleep(1);
}

#[test]
fn default_warning_compiles() {
    #[slow_function_warning(10ms)]
    pub fn sleep(millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    sleep(1);
}

#[test]
fn warn() {
    #[slow_function_warning(10ms, write_to_file("./slow_function_warning_warn.txt", &format!("{module}::{function}")))]
    pub fn sleep(millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    sleep(10);

    assert_eq!(
        read_from_file("./slow_function_warning_warn.txt"),
        "slow_function_warning::sleep"
    );
}

#[test]
fn no_warn() {
    #[slow_function_warning(10ms, write_to_file("./slow_function_warning_no_warn.txt", &format!("{module}::{function}")))]
    pub fn sleep(millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    sleep(1);

    assert!(!file_exists("./slow_function_warning_no_warn.txt"));
}

#[test]
fn warn_using_params() {
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, write_to_file("./slow_function_warning_warn_using_params.txt", &format!("{module}::{function} {param}")))]
    pub fn sleep(millis: u64, param: &str) {
        thread::sleep(Duration::from_millis(millis));
    }

    sleep(10, "trace id");

    assert_eq!(
        read_from_file("./slow_function_warning_warn_using_params.txt"),
        "slow_function_warning::sleep trace id"
    );
}

#[test]
fn no_warn_using_params() {
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, write_to_file("./slow_function_warning_no_warn_using_params.txt", &format!("{module}::{function} {param}")))]
    pub fn sleep(millis: u64, param: &str) {
        thread::sleep(Duration::from_millis(millis));
    }

    sleep(1, "trace id");

    assert!(!file_exists(
        "./slow_function_warning_no_warn_using_params.txt"
    ));
}

#[test]
fn warn_impl() {
    struct MyStruct {
        pub value: u64,
    }

    impl MyStruct {
        #[slow_function_warning(10ms, write_to_file("./slow_function_warning_warn_impl.txt", &format!("{module}::{function} {param}")))]
        pub fn sleep(&mut self, millis: u64, param: &str) {
            println!("{} {millis} {param}", self.value);
            thread::sleep(Duration::from_millis(millis));
        }
    }

    let mut my_struct = MyStruct { value: 10 };
    my_struct.sleep(10, "trace id");

    assert_eq!(
        read_from_file("./slow_function_warning_warn_impl.txt"),
        "slow_function_warning::sleep trace id"
    );
}

#[test]
fn no_warn_impl() {
    struct MyStruct {
        pub value: u64,
    }

    impl MyStruct {
        #[slow_function_warning(10ms, write_to_file("./slow_function_warning_no_warn_impl.txt", &format!("{module}::{function} {param}")))]
        pub fn sleep(&mut self, millis: u64, param: &str) {
            println!("{} {millis} {param}", self.value);
            thread::sleep(Duration::from_millis(millis));
        }
    }

    let mut my_struct = MyStruct { value: 10 };
    my_struct.sleep(1, "trace id");

    assert!(!file_exists("./slow_function_warning_no_warn_impl.txt"));
}

#[tokio::test]
async fn warn_async() {
    #[slow_function_warning(10ms, write_to_file("./slow_function_warning_warn_async.txt", &format!("{module}::{function}")))]
    pub async fn sleep(millis: u64) {
        tokio::time::sleep(Duration::from_millis(millis)).await;
    }

    sleep(10).await;

    assert_eq!(
        read_from_file("./slow_function_warning_warn_async.txt"),
        "slow_function_warning::sleep"
    );
}

#[tokio::test]
async fn no_warn_async() {
    #[slow_function_warning(10ms, write_to_file("./slow_function_warning_no_warn_async.txt", &format!("{module}::{function}")))]
    pub async fn sleep(millis: u64) {
        tokio::time::sleep(Duration::from_millis(millis)).await;
    }

    sleep(1).await;

    assert!(!file_exists("./slow_function_warning_no_warn_async.txt"));
}
