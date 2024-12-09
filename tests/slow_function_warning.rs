use slow_function_warning::*;
use std::{thread, time::Duration};

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
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, {*warned = true;})]
    pub fn sleep(millis: u64, warned: &mut bool) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut warned = false;
    sleep(100, &mut warned);

    assert!(warned);
}

#[test]
fn no_warn() {
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, {*warned = true;})]
    pub fn sleep(millis: u64, warned: &mut bool) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut warned = false;
    sleep(1, &mut warned);

    assert!(!warned);
}

#[test]
fn warn_using_params() {
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, {
        println!("{module}::{function} {param}");
        *warned = true;
    })]
    pub fn sleep(millis: u64, param: &str, warned: &mut bool) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut warned = false;
    sleep(10, "trace id", &mut warned);

    assert!(warned);
}

#[test]
fn no_warn_using_params() {
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, {
        println!("{module}::{function} {param}");
        *warned = true;
    })]
    pub fn sleep(millis: u64, param: &str, warned: &mut bool) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut warned = false;
    sleep(1, "trace id", &mut warned);

    assert!(!warned);
}

#[test]
fn warn_impl() {
    struct MyStruct {
        pub warned: bool,
    }

    impl MyStruct {
        #[allow(unused_variables)]
        #[slow_function_warning(10ms, {
            println!("{module}::{function} {param}");
            self.warned = true;
        })]
        pub fn sleep(&mut self, millis: u64, param: &str) {
            thread::sleep(Duration::from_millis(millis));
        }
    }

    let mut my_struct = MyStruct { warned: false };
    my_struct.sleep(10, "trace id");

    assert!(my_struct.warned);
}

#[test]
fn no_warn_impl() {
    struct MyStruct {
        pub warned: bool,
    }

    impl MyStruct {
        #[allow(unused_variables)]
        #[slow_function_warning(10ms, {
            println!("{module}::{function} {param}");
            self.warned = true;
        })]
        pub fn sleep(&mut self, millis: u64, param: &str) {
            thread::sleep(Duration::from_millis(millis));
        }
    }

    let mut my_struct = MyStruct { warned: false };
    my_struct.sleep(1, "trace id");

    assert!(!my_struct.warned);
}

#[tokio::test]
async fn warn_async() {
    #[allow(unused_variables)]
    #[slow_function_warning(10ms, {
        println!("{module}::{function} {param}");
        *warned = true;
    })]
    pub async fn sleep(millis: u64, param: &str, warned: &mut bool) {
        tokio::time::sleep(Duration::from_millis(millis)).await;
    }

    let mut warned = false;
    sleep(10, "trace id", &mut warned).await;

    assert!(warned);
}

#[tokio::test]
async fn no_warn_async() {
    #[allow(unused_variables)]
    #[slow_function_warning(50ms, {
        println!("{module}::{function} {param}");
        *warned = true;
    })]
    pub async fn sleep(millis: u64, param: &str, warned: &mut bool) {
        tokio::time::sleep(Duration::from_millis(millis)).await;
    }

    let mut warned = false;
    sleep(1, "trace id", &mut warned).await;

    assert!(!warned);
}
