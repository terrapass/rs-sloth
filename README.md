[![crates.io](http://meritbadge.herokuapp.com/sloth)](https://crates.io/crates/sloth)
[![docs.rs](https://docs.rs/sloth/badge.svg)](https://docs.rs/sloth)
[![Build Status](https://travis-ci.org/terrapass/rs-sloth.svg?branch=master)](https://travis-ci.org/terrapass/rs-sloth)

# sloth

This crate provides a generic [`Lazy<T, Eval>`](https://docs.rs/sloth/0.1.2/sloth/struct.Lazy.html) wrapper struct for lazy initialization.
It can be used for expensive-to-calculate `T` values to ensure that the evaluation logic runs
only once and only if needed.

For example:
```rust
use sloth::Lazy;

fn get_expensive_string() -> String {
    // do something expensive here to obtain the result,
    // such as read and process file contents

    String::from("some expensive string we got from a file or something")
}

fn get_expensive_i32() -> i32 {
    // do something expensive here to calculate the result,
    // such as build a supercomputer and wait 7.5 million years

    42
}

let lazy_string = Lazy::new(get_expensive_string);
let lazy_i32 = Lazy::new(get_expensive_i32);

//...

let must_use_string = true;

//...

if must_use_string {
    println!("Expensive string is: {}", *lazy_string.value_ref());
    println!("It has length: {}", (*lazy_string.value_ref()).len());

    // get_expensive_string() has been called only once,
    // get_expensive_i32() has not been called at all
} else {
    println!("Expensive int is: {}", lazy_i32.value());
    println!("It is{} divisible by 6", if lazy_i32.value() % 6 == 0 { "" } else { " not" });

    // get_expensive_string() has not been called,
    // get_expensive_i32() has been called only once
}

```

Current version: 0.1.2
