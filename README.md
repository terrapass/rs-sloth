[![crates.io](http://meritbadge.herokuapp.com/sloth)](https://crates.io/crates/sloth)
[![docs.rs](https://docs.rs/sloth/badge.svg)](https://docs.rs/sloth)
[![Build Status](https://travis-ci.org/terrapass/rs-sloth.svg?branch=master)](https://travis-ci.org/terrapass/rs-sloth)

# sloth

This crate provides a generic pointer-like [`Lazy<T, Eval>`](https://docs.rs/sloth/0.2.0/sloth/struct.Lazy.html) struct for lazily initialized values.
It can be used for expensive-to-calculate values to ensure that the evaluation logic runs
only once and only if needed.

For example:
```rust
use sloth::Lazy;

fn get_expensive_string() -> String {
    // do something expensive here to obtain the result,
    // such as read and process file contents
    String::from("some expensive string we got from a file or something")
}

fn get_expensive_number() -> i32 {
    // do something expensive here to calculate the result,
    // such as build a supercomputer and wait 7.5 million years
    42
}

let lazy_string = Lazy::new(get_expensive_string);
let lazy_number = Lazy::new(get_expensive_number);

//...
let must_use_string = true;
//...

if must_use_string {
    println!("Expensive string is: {}", *lazy_string);
    println!("It has length: {}", lazy_string.len());

    // get_expensive_string() has been called only once,
    // get_expensive_number() has not been called
} else {
    println!("Expensive number is: {}", *lazy_number);
    println!("Its square is {}", lazy_number.pow(2));

    // get_expensive_string() has not been called,
    // get_expensive_number() has been called only once
}

```

The evaluated value of a mutable [`Lazy`](https://docs.rs/sloth/0.2.0/sloth/struct.Lazy.html) can be modified:
```rust
use sloth::Lazy;

let mut lazy_vec = Lazy::new(|| vec![2, -5, 6, 0]);

lazy_vec.retain(|n| *n > 0);

assert_eq!(*lazy_vec, vec![2, 6]);
```

[`Lazy`](https://docs.rs/sloth/0.2.0/sloth/struct.Lazy.html) can be consumed and turned into its value via [`unwrap()`](https://docs.rs/sloth/0.2.0/sloth/struct.Lazy.html#method.unwrap):
```rust
use sloth::Lazy;

let lazy_value = Lazy::new(|| "moo");

let output = String::from("a cow goes ") + lazy_value.unwrap();
```
