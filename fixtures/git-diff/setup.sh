#!/usr/bin/env bash
# Setup: create a git repo with staged and unstaged changes to diff.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

git init -b main -q
git config user.email "demo@litmus.dev"
git config user.name "Litmus Demo"

# Create initial files and make a commit
mkdir -p src

cat > src/main.rs << 'RUST'
use std::collections::HashMap;

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn count_words(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let message = greet("world");
    println!("{}", message);

    let text = "the quick brown fox jumps over the lazy dog";
    let counts = count_words(text);
    println!("Word count: {}", counts.len());
}
RUST

cat > src/lib.rs << 'RUST'
pub mod utils;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
RUST

GIT_AUTHOR_DATE="2024-01-15T10:00:00" \
GIT_COMMITTER_DATE="2024-01-15T10:00:00" \
git add -A
GIT_AUTHOR_DATE="2024-01-15T10:00:00" \
GIT_COMMITTER_DATE="2024-01-15T10:00:00" \
git commit -q -m "Initial commit"

# Now modify files to create a diff
cat > src/main.rs << 'RUST'
use std::collections::HashMap;
use std::io::{self, BufRead};

fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to litmus.", name)
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines().filter_map(|l| l.ok()).collect()
}

fn main() {
    let message = greet("world");
    println!("{}", message);

    let text = "the quick brown fox jumps over the lazy dog";
    let counts = count_words(text);
    println!("Unique words: {}", counts.len());
    println!("Total words: {}", text.split_whitespace().count());
}
RUST

cat > src/lib.rs << 'RUST'
pub mod utils;
pub mod config;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// subtract is deprecated, use std::ops::Sub instead
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
RUST
