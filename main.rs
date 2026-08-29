---cargo
[package]
edition = "2024"
[dependencies]
minreq = "3.0.0"
---

use std::env;
use std::fs;

fn main() {
    let skip_dirs = env::args().nth(1);
    let mut matcher_path = env::var("GITHUB_ACTION_PATH").unwrap();
    matcher_path.push_str("/gcc_matcher.json");

    let mut gcc_matcher = fs::read_to_string(&matcher_path).unwrap();

    if let Some(dirs) = skip_dirs {
        let escaped = escape_chars(&dirs);
        let formatted = format_strings(&escaped);
        gcc_matcher = gcc_matcher.replace("{{SKIP_DIRS}}", &formatted);
    } else {
        gcc_matcher = gcc_matcher.replace("(?!(?:{{SKIP_DIRS}}))", ""); //Remove negative lookahead entirely
    }

    
    fs::write(&matcher_path, &gcc_matcher).unwrap();
    println!("Full matcher json: {}", gcc_matcher);

    let mut event_name = env::var("GITHUB_EVENT_NAME").unwrap();
    if event_name == "push" {
        println!("::add-matcher::{}", matcher_path);
    }
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

fn format_strings(s: &str) -> String {
    let mut buffer = String::new();
    let mut lines = s.lines().peekable();

    while let Some(l) = lines.next() {        
        if !l.starts_with('/') {
            buffer.push('/');
        }

        buffer.push_str(l);

        if !l.ends_with('/') {
            buffer.push('/');
        }

        if lines.peek().is_some() {
            buffer.push('|');
        }
    }
    buffer
}
