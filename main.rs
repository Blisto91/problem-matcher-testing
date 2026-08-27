#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"
[dependencies]
---

use std::env;
  
fn main() {
    let path = env::var("GITHUB_ACTION_PATH").unwrap();
    println!("::add-matcher::{}/gcc_matcher.json", path);
}
