#!/usr/bin/env bash
# Setup: create a minimal Rust project that produces warnings and one error.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

mkdir -p src

cat > Cargo.toml << 'TOML'
[package]
name = "demo"
version = "0.1.0"
edition = "2021"
TOML

cat > src/main.rs << 'RUST'
// Intentional: unused variable warning, dead code warning, type error
fn compute(x: i32, y: i32) -> i32 {
    let unused_result = x * 2;
    x + y
}

fn deprecated_helper(val: f64) -> f64 {
    val * 3.14
}

fn main() {
    let result = compute(10, 20);
    println!("Result: {}", result);

    // Type error: expected i32, found &str
    let _bad: i32 = "not a number";
}
RUST
