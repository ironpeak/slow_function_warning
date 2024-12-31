use std::{thread, time::Duration};

use slow_function_warning::*;

#[test]
fn warn() {
    #[slow_function_warning(1ms, {*warned = true;})]
    pub fn sleep(millis: u64, warned: &mut bool) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut warned = false;
    sleep(2, &mut warned);

    assert!(warned);
}

#[test]
fn no_warn() {
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
    #[slow_function_warning(1ms, {
        println!("{module}::{function} {param}");
        *warned = true;
    })]
    pub fn sleep(millis: u64, param: &str, warned: &mut bool) {
        thread::sleep(Duration::from_millis(millis));
    }

    let mut warned = false;
    sleep(2, "trace id", &mut warned);

    assert!(warned);
}

#[test]
fn no_warn_using_params() {
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
        #[slow_function_warning(1ms, {
            println!("{module}::{function} {param}");
            self.warned = true;
        })]
        pub fn sleep(&mut self, millis: u64, param: &str) {
            thread::sleep(Duration::from_millis(millis));
        }
    }

    let mut my_struct = MyStruct { warned: false };
    my_struct.sleep(2, "trace id");

    assert!(my_struct.warned);
}

#[test]
fn no_warn_impl() {
    struct MyStruct {
        pub warned: bool,
    }

    impl MyStruct {
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
    #[slow_function_warning(1ms, {
        println!("{module}::{function} {param}");
        *warned = true;
    })]
    pub async fn sleep(millis: u64, param: &str, warned: &mut bool) {
        tokio::time::sleep(Duration::from_millis(millis)).await;
    }

    let mut warned = false;
    sleep(2, "trace id", &mut warned).await;

    assert!(warned);
}

#[tokio::test]
async fn no_warn_async() {
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
