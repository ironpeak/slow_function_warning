# Slow Function Warning

This crate provides a procedural macro to inject timers into functions and print a warning if it takes longer than expected. It can be particularly useful for debugging performance issues during development.

This is not meant to be a benchmarking tool, but rather a way to detect potential performance issues in your code.

For example my use case was for developing a game in [Bevy](https://bevyengine.org/) and I've added this to all my systems to detect if any game system function takes longer than a 1ms.

# Usage

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
slow_function_warning = "0.2.0"

# For wasm targets
[target.'cfg(target_family = "wasm")'.dependencies]
web-time = "1"
```

## Basic Example

```rust
#[slow_function_warning(1000ms)] // Warn if the function takes longer than 1000 milliseconds
fn example_function() {
    // Function implementation
}
```

## Debug Only Example

```rust
#[cfg_attr(debug_assertions, slow_function_warning(1000ms))]
fn example_function() {
    // Function implementation
}
```

Or using the convenience proc macro:

```rust
#[debug_slow_function_warning(1000ms)]
fn example_function() {
    // Function implementation
}
```

## Release Only Example

```rust
#[cfg_attr(not(debug_assertions), slow_function_warning(1000ms))]
fn example_function() {
    // Function implementation
}
```

Or using the convenience proc macro:

```rust
#[release_slow_function_warning(1000ms)]
fn example_function() {
    // Function implementation
}
```

## Custom Message Example

```rust
// Warn if the function takes longer than a second with a custom message
#[debug_slow_function_warning(1s, println!("Function {function} took too long!"))]
fn example_function() {
    // Function implementation
}
```

You can also use the function parameters in your message:

```rust
// Warn if the function takes longer than a second with a custom message
#[debug_slow_function_warning(1s, println!("Function {function} took {millis} for {} values!", values.len()))]
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
* `nanos: u64` - The elapsed time in nanoseconds
* `ns: u64` - The elapsed time in nanoseconds
* `millis: u64` - The elapsed time in milliseconds
* `ms: u64` - The elapsed time in milliseconds
* `secs: u64` - The elapsed time in seconds
* `s: u64` - The elapsed time in seconds

## How it works

This is a procedural macro that takes the content of a function and places it in a closure, executes it and times how long it took.

```rust
// Warn if the function takes longer than a second with a custom message
#[debug_slow_function_warning(1s, println!("Function {function} took too long!"))]
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
        let module = "module name";
        let function = "example_function";
        let elapsed = start.elapsed();
        let ns = elapsed.as_nanos();
        let nanos = ns;
        let ms = elapsed.as_millis();
        let millis = ms;
        let s = elapsed.as_secs();
        let secs = s;
        println!("Function {function} took too long!")
    }
    result
}
```
