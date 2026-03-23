#![allow(dead_code)]
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("provider not found: {0}")]
    ProviderNotFound(String),

    #[error("theme not found: {0}")]
    ThemeNotFound(String),

    #[error("fixture not found at {0}")]
    FixtureNotFound(String),

    #[error("fixture script failed: {script} exited with {code}")]
    FixtureScriptFailed { script: String, code: i32 },

    #[error("timeout waiting for fixture to complete after {0}s")]
    FixtureTimeout(u64),

    #[error("screenshot tool failed: {0}")]
    ScreenshotFailed(String),

    #[error("image conversion failed: {0}")]
    ImageConversionFailed(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
