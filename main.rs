---cargo
[package]
edition = "2024"
---

use std::env;
  
fn main() {
    let skip_dirs = env::args().nth(1).expect("no skip-directories argument given");
    let path = env::var("GITHUB_ACTION_PATH").unwrap();
    let gcc_matcher = fs::read_to_string(file_path).unwrap();

    if skip_dirs != "" {
        gcc_matcher.replace("{{SKIP_DIRS}}", skip_dirs);
    }

    println!("::add-matcher::{}/gcc_matcher.json", path);
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
