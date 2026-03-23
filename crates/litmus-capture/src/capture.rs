use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

use crate::error::CaptureError;
use crate::providers::{ProviderCapture, TermGeometry};

/// Options for a single screenshot capture.
pub struct CaptureOptions<'a> {
    pub provider: &'a dyn ProviderCapture,
    pub theme: &'a litmus_model::Theme,
    /// Path to the fixture directory (contains setup.sh, command.sh)
    pub fixture_dir: &'a Path,
    /// Fixture id used for logging (e.g. "git-diff")
    pub fixture_id: &'a str,
    /// Where to write the output image
    pub output_path: &'a Path,
    pub geometry: TermGeometry,
    /// Maximum seconds to wait for the fixture command to complete
    pub timeout_secs: u64,
}

/// Result of a successful capture.
pub struct CaptureResult {
    pub width: u32,
    pub height: u32,
    /// SHA-256 hex digest of the output image
    pub checksum: String,
}

/// Capture a single screenshot.
///
/// Flow:
/// 1. Create temporary directories for: provider config, fixture work dir, sentinel file
/// 2. Write provider config (theme + geometry settings)
/// 3. Run fixture `setup.sh` with FIXTURE_WORK_DIR set
/// 4. Write a capture wrapper script that:
///    a. Launches the terminal with the command
///    b. Waits for the sentinel file (written after command exits)
///    c. Takes a screenshot with grim
///    d. Kills the terminal and exits
/// 5. Run `cage -- bash wrapper.sh` and wait for it to exit
/// 6. Convert PNG → WebP
/// 7. Compute checksum and return
pub fn capture_screenshot(opts: &CaptureOptions) -> Result<CaptureResult> {
    let tmp = tempfile::TempDir::new().context("create temp dir")?;

    // Write provider config
    let config_content = opts
        .provider
        .generate_config(opts.theme, &opts.geometry);
    let config_ext = opts.provider.config_extension();
    let config_path = tmp.path().join(format!("provider.{}", config_ext));
    fs::write(&config_path, &config_content).context("write provider config")?;

    // Create fixture work dir
    let work_dir = tmp.path().join("fixture-work");
    fs::create_dir_all(&work_dir).context("create fixture work dir")?;

    // Run fixture setup.sh
    let setup_script = opts.fixture_dir.join("setup.sh");
    if setup_script.exists() {
        let status = Command::new("bash")
            .arg(&setup_script)
            .env("FIXTURE_WORK_DIR", &work_dir)
            .status()
            .context("run setup.sh")?;
        if !status.success() {
            return Err(CaptureError::FixtureScriptFailed {
                script: "setup.sh".to_string(),
                code: status.code().unwrap_or(-1),
            }
            .into());
        }
    }

    // Sentinel file: command.sh will touch this when done
    let sentinel_path = tmp.path().join("done.sentinel");
    let png_path = tmp.path().join("screenshot.png");

    // Build the command that the terminal runs: execute command.sh, touch sentinel, then sleep
    // Canonicalize to absolute path so the script is findable after `cd` to the work dir
    let command_script = fs::canonicalize(opts.fixture_dir.join("command.sh"))
        .context("resolve command.sh path")?;
    let _ = opts.fixture_id; // used for identification by callers
    let work_dir_abs = fs::canonicalize(&work_dir).context("resolve work dir")?;
    let terminal_command = format!(
        "export FIXTURE_WORK_DIR='{}' && cd '{}' && bash '{}'; touch '{}'; sleep 300",
        work_dir_abs.display(),
        work_dir_abs.display(),
        command_script.display(),
        sentinel_path.display(),
    );

    // Build terminal launch args
    let terminal_args = opts
        .provider
        .build_launch_args(&config_path, &terminal_command);
    let terminal_bin = terminal_args
        .first()
        .context("terminal args must not be empty")?;

    // Write the cage wrapper script
    let wrapper_content = build_wrapper_script(
        terminal_bin,
        &terminal_args[1..],
        &sentinel_path,
        &png_path,
        opts.timeout_secs,
    );
    let wrapper_path = tmp.path().join("capture-wrapper.sh");
    fs::write(&wrapper_path, &wrapper_content).context("write wrapper script")?;

    // Run the wrapper inside cage (headless Wayland compositor).
    // WLR_BACKENDS=headless creates a headless Wayland display without DRM.
    // WLR_RENDERER=pixman uses software rendering; foot (used for capture) renders via SHM so no OpenGL is needed.
    let cage_status = Command::new("cage")
        .args(["--"])
        .arg("bash")
        .arg(&wrapper_path)
        .env("WLR_BACKENDS", "headless")
        .env("WLR_RENDERER", "pixman")
        .env("WLR_SILENT", "1")
        .status()
        .context("run cage")?;

    if !cage_status.success() {
        // cage exits non-zero if the inner program exits non-zero; check if we got a screenshot
        if !png_path.exists() {
            bail!(
                "cage exited with {:?} and no screenshot was produced",
                cage_status.code()
            );
        }
    }

    if !png_path.exists() {
        bail!("cage exited but screenshot file was not created at {}", png_path.display());
    }

    // Convert PNG → WebP
    let webp_bytes = png_to_webp(&png_path).context("convert PNG to WebP")?;
    let (width, height) = png_dimensions(&png_path).context("read PNG dimensions")?;

    // Write output
    if let Some(parent) = opts.output_path.parent() {
        fs::create_dir_all(parent).context("create output directory")?;
    }
    fs::write(opts.output_path, &webp_bytes).context("write WebP output")?;

    // Compute checksum of the final WebP
    let checksum = sha256_hex(&webp_bytes);

    Ok(CaptureResult { width, height, checksum })
}

/// Build the bash wrapper script that runs inside cage.
///
/// The script:
/// 1. Launches the terminal emulator in the background
/// 2. Polls for the sentinel file (indicating the fixture command finished)
/// 3. Takes a screenshot with grim
/// 4. Kills the terminal and exits
fn build_wrapper_script(
    terminal_bin: &str,
    terminal_extra_args: &[String],
    sentinel_path: &Path,
    png_output: &Path,
    timeout_secs: u64,
) -> String {
    let extra_args = terminal_extra_args
        .iter()
        .map(|a| shell_escape(a))
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        r#"#!/usr/bin/env bash
set -euo pipefail

# Launch terminal emulator in background (WAYLAND_DISPLAY is set by cage)
{terminal_bin} {extra_args} &
TERMINAL_PID=$!

# Poll for sentinel file (written when fixture command completes)
TIMEOUT={timeout}
ELAPSED=0
while [[ ! -f {sentinel} ]]; do
    sleep 0.2
    ELAPSED=$((ELAPSED + 1))
    if [[ $ELAPSED -ge $((TIMEOUT * 5)) ]]; then
        echo "litmus-capture: timeout waiting for fixture after {timeout}s" >&2
        kill $TERMINAL_PID 2>/dev/null || true
        exit 1
    fi
done

# Extra wait for rendering to fully settle
sleep 0.5

# Take screenshot of the entire Wayland display (grim uses $WAYLAND_DISPLAY set by cage)
grim {png_output}

# Kill terminal and exit cleanly
kill $TERMINAL_PID 2>/dev/null || true
wait $TERMINAL_PID 2>/dev/null || true
"#,
        terminal_bin = terminal_bin,
        extra_args = extra_args,
        timeout = timeout_secs,
        sentinel = shell_escape(sentinel_path.to_str().expect("sentinel path must be valid UTF-8")),
        png_output = shell_escape(png_output.to_str().expect("png path must be valid UTF-8")),
    )
}

/// Very basic shell escaping: wrap in single quotes, escape embedded single quotes.
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Convert a PNG file to WebP lossless bytes using the `image` crate.
fn png_to_webp(png_path: &Path) -> Result<Vec<u8>> {
    let img = image::open(png_path).context("open PNG")?;
    let mut buf = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::WebP,
    )
    .context("encode WebP")?;
    Ok(buf)
}

/// Read pixel dimensions from a PNG file without decoding all pixel data.
fn png_dimensions(png_path: &Path) -> Result<(u32, u32)> {
    let reader = image::ImageReader::open(png_path).context("open PNG for dimensions")?;
    let (w, h) = reader.into_dimensions().context("read image dimensions")?;
    Ok((w, h))
}

/// Compute SHA-256 hex digest of a byte slice.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_escape_simple() {
        assert_eq!(shell_escape("hello"), "'hello'");
    }

    #[test]
    fn shell_escape_with_spaces() {
        assert_eq!(shell_escape("/tmp/my dir/file.conf"), "'/tmp/my dir/file.conf'");
    }

    #[test]
    fn shell_escape_with_single_quotes() {
        assert_eq!(shell_escape("it's"), "'it'\\''s'");
    }

    #[test]
    fn sha256_hex_known_value() {
        // SHA-256 of empty string
        let result = sha256_hex(b"");
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn wrapper_script_contains_key_parts() {
        let sentinel = Path::new("/tmp/sentinel");
        let png = Path::new("/tmp/out.png");
        let script = build_wrapper_script("kitty", &["--config".to_string(), "/tmp/k.conf".to_string()], sentinel, png, 30);

        assert!(script.contains("kitty"));
        assert!(script.contains("--config"));
        assert!(script.contains("/tmp/sentinel"));
        assert!(script.contains("/tmp/out.png"));
        assert!(script.contains("grim"));
        assert!(script.contains("TIMEOUT=30"));
    }
}
