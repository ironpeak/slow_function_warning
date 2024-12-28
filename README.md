# Slow Function Warning

This crate provides a procedural macro to inject timers into functions and print a warning if it takes longer than expected. It can be particularly useful for debugging performance issues during development.

This is not meant to be a benchmarking tool, but rather a way to detect potential performance issues in your code.

For example my use case was for developing a game in [Bevy](https://bevyengine.org/) and I've added this to all my systems to detect if any game system function takes longer than a 1ms.

# Usage

## Installation

Add the following to your `Cargo.toml`:

```toml
[features]
slow_function_warning = ["dep:slow_function_warning"]

[dependencies]
# Add as a feature to avoid affecting the LSP.
slow_function_warning = { version = "0.5.0", optional = true }

# For wasm targets
[target.'cfg(target_family = "wasm")'.dependencies]
web-time = "1"
```

## Basic Example

```rust
#[cfg_attr(feature = "slow_function_warning", slow_function_warning(1000ms))] // Warn if the function takes longer than 1000 milliseconds
fn example_function() {
    // Function implementation
}
```

The warning is not on by default and is only recommended for debugging purposes. To enable it use the `slow_function_warning` feature.

~~~bash
cargo run --features slow_function_warning
~~~

## Custom Message Example

```rust
// Warn if the function takes longer than a second with a custom message
#[cfg_attr(feature = "slow_function_warning", slow_function_warning(1ms, println!("Function {function} took too long!")))]
fn example_function() {
    // Function implementation
}
```

You can also use the function parameters in your message:

```rust
// Warn if the function takes longer than a second with a custom message
#[cfg_attr(feature = "slow_function_warning", slow_function_warning(1s, println!("Function {function} took {millis} for {} values!", values.len())))]
fn sort(values: &Vec<u32>) {
    // Function implementation
}
```

## Duration Syntax

You can specify the duration using numeric literals followed by a suffix:

* `ns` for nanoseconds
* `ms` for milliseconds
* `s` for seconds
* `m` for minutes
* `h` for hours
* `d` for days

## Available Variables

* `module: String` - The name of the module
* `function: String` - The name of the function
* `elapsed: Duration` - The elapsed time
* `elapsed_str: String` - The elapsed time using the limit unit specified (defaults to milliseconds)
* `elapsed_ns: u128` - The elapsed time in nanoseconds
* `elapsed_nanos: u128` - The elapsed time in nanoseconds
* `elapsed_nanoseconds: u128` - The elapsed time in nanoseconds
* `elapsed_ms: u128` - The elapsed time in milliseconds
* `elapsed_millis: u128` - The elapsed time in milliseconds
* `elapsed_milliseconds: u128` - The elapsed time in milliseconds
* `elapsed_s: u64` - The elapsed time in seconds
* `elapsed_secs: u64` - The elapsed time in seconds
* `elapsed_seconds: u64` - The elapsed time in seconds
* `elapsed_m: u64` - The elapsed time in minutes
* `elapsed_min: u64` - The elapsed time in minutes
* `elapsed_minutes: u64` - The elapsed time in minutes
* `elapsed_h: u64` - The elapsed time in hours
* `elapsed_hours: u64` - The elapsed time in hours
* `elapsed_d: u64` - The elapsed time in days
* `elapsed_days: u64` - The elapsed time in days
* `limit: Duration` - The name of the module
* `limit_str: String` - The limit time using the limit unit specified (defaults to milliseconds)
* `limit_ns: u128` - The limit time in nanoseconds
* `limit_nanos: u128` - The limit time in nanoseconds
* `limit_nanoseconds: u128` - The limit time in nanoseconds
* `limit_ms: u128` - The limit time in milliseconds
* `limit_millis: u128` - The limit time in milliseconds
* `limit_milliseconds: u128` - The limit time in milliseconds
* `limit_s: u64` - The limit time in seconds
* `limit_secs: u64` - The limit time in seconds
* `limit_seconds: u64` - The limit time in seconds
* `limit_m: u64` - The limit time in minutes
* `limit_min: u64` - The limit time in minutes
* `limit_minutes: u64` - The limit time in minutes
* `limit_h: u64` - The limit time in hours
* `limit_hours: u64` - The limit time in hours
* `limit_d: u64` - The limit time in days
* `limit_days: u64` - The limit time in days

## How it works

This is a procedural macro that takes the content of a function and places it in a closure, executes it and times how long it took.

```rust
// Warn if the function takes longer than a second with a custom message
#[cfg_attr(feature = "slow_function_warning", slow_function_warning(1s, println!("Function {function} took too long!")))]
fn example_function() {
    let x = 10;
}
```

Becomes:

```rust
fn example_function() {
    let closure = || {
        let x = 10;
    };
    #[cfg(not(target_family = "wasm"))]
    let start = std::time::Instant::now();
    #[cfg(target_family = "wasm")]
    let start = web_time::Instant::now();
    let result = closure();
    if start.elapsed().as_nanos() > 1000000 {
        let module = module_path!();
        let function = #function_name;

        let elapsed = start.elapsed();
        let elapsed_str = #elapsed_str;
        let elapsed_ns = elapsed.as_nanos();
        let elapsed_nanos = elapsed_ns;
        let elapsed_nanoseconds = elapsed_ns;
        let elapsed_ms = elapsed.as_millis();
        let elapsed_millis = elapsed_ms;
        let elapsed_milliseconds = elapsed_ms;
        let elapsed_s = elapsed.as_secs();
        let elapsed_secs = elapsed_s;
        let elapsed_seconds = elapsed_s;
        let elapsed_m = elapsed.as_secs() / 60;
        let elapsed_min = elapsed_m;
        let elapsed_minutes = elapsed_m;
        let elapsed_h = elapsed.as_secs() / 60 / 60;
        let elapsed_hours = elapsed_h;
        let elapsed_d = elapsed.as_secs() / 60 / 60 / 24;
        let elapsed_days = elapsed_d;

        let limit = Duration::from_nanos(#nano_seconds as u64);
        let limit_str = #limit_str;
        let limit_ns = limit.as_nanos();
        let limit_nanos = limit_ns;
        let limit_nanoseconds = limit_ns;
        let limit_ms = limit.as_millis();
        let limit_millis = limit_ms;
        let limit_milliseconds = limit_ms;
        let limit_s = limit.as_secs();
        let limit_secs = limit_s;
        let limit_seconds = limit_s;
        let limit_m = limit.as_secs() / 60;
        let limit_min = limit_m;
        let limit_minutes = limit_m;
        let limit_h = limit.as_secs() / 60 / 60;
        let limit_hours = limit_h;
        let limit_d = limit.as_secs() / 60 / 60 / 24;
        let limit_days = limit_d;

        println!("Warning: {module}::{function}: ran for {elapsed_str} (limit: {limit_str})")
    }
    result
}
```
