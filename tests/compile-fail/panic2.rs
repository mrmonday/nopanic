// Copyright (c) 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: error: this expression could cause a panic!
// compile-flags: -g

#![feature(custom_attribute, plugin)]
#![allow(dead_code)]
#![plugin(nopanic)]

#[nopanic]
fn panic2() {
    let a = 0;
    3 / a;
}

fn main() {
    panic2();
}

