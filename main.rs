---cargo
[package]
edition = "2024"
---

use std::env;
use std::fs;
  
fn main() {
    let skip_dirs = env::args().nth(1).expect("no skip-directories argument given");
    let path = env::var("GITHUB_ACTION_PATH").unwrap();
    
    let old_gcc_matcher = fs::read_to_string("gcc_matcher.json").unwrap();
    let new_gcc_matcher = gcc_matcher.replace("{{SKIP_DIRS}}", &skip_dirs);
    fs::write("gcc_matcher.json", new_gcc_matcher).unwrap();

    println!("::add-matcher::{}/gcc_matcher.json", path).unwrap();
}

fn escape_chars (s: &str) -> String {
    let meta_chars = String::from(r"\+-?.*^$|()[]{}");
    let mut buffer = String::new();

    for c in s.chars() {
        if meta_chars.contains(c) {
            buffer.extend(['\\', c]);
        } else {
            buffer.push(c);
        }
    }
    buffer
}
