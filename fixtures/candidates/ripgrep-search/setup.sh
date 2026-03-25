#!/usr/bin/env bash
# Setup: create a small Rust project to search through with ripgrep.
set -euo pipefail

cd "$FIXTURE_WORK_DIR"

mkdir -p src

cat > src/main.rs << 'RUST'
use std::collections::HashMap;
use std::io::{self, BufRead};

/// Parse a configuration file into key-value pairs.
fn parse_config(input: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    config
}

/// Validate that required config keys are present.
fn validate_config(config: &HashMap<String, String>) -> Result<(), Vec<String>> {
    let required = ["host", "port", "database", "timeout"];
    let missing: Vec<String> = required
        .iter()
        .filter(|k| !config.contains_key(**k))
        .map(|k| k.to_string())
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(missing)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().filter_map(|l| l.ok()).collect();
    let config = parse_config(&input);

    match validate_config(&config) {
        Ok(()) => println!("Config valid: {} keys", config.len()),
        Err(missing) => eprintln!("Missing keys: {}", missing.join(", ")),
    }

    Ok(())
}
RUST

cat > src/lib.rs << 'RUST'
pub mod config;
pub mod error;

/// Application-wide configuration defaults.
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 5432;
pub const DEFAULT_TIMEOUT: u64 = 30;

/// Connection configuration.
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub timeout: u64,
}

impl Config {
    pub fn new(database: &str) -> Self {
        Config {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            database: database.to_string(),
            timeout: DEFAULT_TIMEOUT,
        }
    }

    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn connection_string(&self) -> String {
        format!("{}:{}/{}", self.host, self.port, self.database)
    }
}
RUST

cat > src/config.rs << 'RUST'
use std::path::Path;

/// Load config from a file path.
pub fn load_config(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Default config file locations to search.
pub fn config_search_paths() -> Vec<String> {
    vec![
        ".config".to_string(),
        "config.toml".to_string(),
        "/etc/app/config.toml".to_string(),
    ]
}
RUST

cat > src/error.rs << 'RUST'
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ConfigNotFound(String),
    InvalidConfig(String),
    ConnectionFailed { host: String, port: u16 },
    Timeout(u64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigNotFound(path) => write!(f, "Config not found: {}", path),
            AppError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            AppError::ConnectionFailed { host, port } => {
                write!(f, "Connection failed: {}:{}", host, port)
            }
            AppError::Timeout(secs) => write!(f, "Timeout after {}s", secs),
        }
    }
}
RUST
