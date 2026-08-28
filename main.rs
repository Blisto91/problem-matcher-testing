---cargo
[package]
edition = "2024"
---

use std::env;
use std::fs;

fn main() {
    let skip_dirs = env::args().nth(1);
    let mut matcher_path = env::var("GITHUB_ACTION_PATH").unwrap();
    matcher_path.push_str("/gcc_matcher.json");

    let mut gcc_matcher = fs::read_to_string(&matcher_path).unwrap();

    if let Some(dirs) = skip_dirs {
        gcc_matcher = gcc_matcher.replace("{{SKIP_DIRS}}", &escape_chars(&dirs));
    } else {
        gcc_matcher = gcc_matcher.replace("{{SKIP_DIRS}}", "//");
    }

    fs::write(matcher_path, gcc_matcher).unwrap();

    println!("::add-matcher::{}", matcher_path);
}

fn escape_chars(s: &str) -> String {
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
