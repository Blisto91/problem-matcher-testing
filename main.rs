#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
---

use std::env;
  
fn main() {
    let path = env::current_dir().unwrap().into_string().unwrap();
    println!("::add-matcher::{}/gcc_matcher.json", path);
}
