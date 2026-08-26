#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
---

use std::env;
  
fn main() {
    let path = env!("GITHUB_ACTION_PATH");
    println!("::add-matcher::{}/gcc_matcher.json", path);
}
