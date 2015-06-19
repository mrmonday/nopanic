# No panic! Rust plugin and lint

This crate provides a plugin for rustc which prevents compilation if the
function, or anything which it calls, could panic.

## Limitations/TODO

There are currently a number of limitations to the plugin:

 * It doesn't traverse into other functions which are called which could panic
 * It doesn't account for panicking which is caused by codegen rather than the
   frontend. This includes:
     - Panicking on array out of bounds
     - Panicking on integer overflow eg. `let a = 0; 4 / a;`
     - Anything else from `grep -r trans_fail librustc_trans/`

## Usage

```rust
#![feature(custom_attribute, plugin)]
#![plugin(nopanic)]

#[nopanic]
pub fn nothing() {
    // This will compile fine
}

#[nopanic]
pub fn panic() {
    panic!("This will cause a compile error");
}
```

Add this to your Cargo.toml:

```
[dependencies.pnet]
git = "https://github.com/mrmonday/nopanic.git"
```

Note that this requires a nightly version of Rust.
