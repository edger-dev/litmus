mod capture;
mod error;
mod manifest;
mod providers;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use litmus_model::screenshot::{Fixture, Provider, ScreenshotManifest};
use litmus_model::{kitty::parse_kitty_theme, toml_format::parse_toml_theme, Theme};

use crate::capture::{capture_screenshot, CaptureOptions};
use crate::manifest::{build_manifest_from_staging, CoverageReport};
use crate::providers::{all_providers, find_provider, TermGeometry};

#[derive(Parser)]
#[command(name = "litmus-capture", about = "Capture real terminal screenshots for litmus themes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture a single screenshot: one provider + one theme + one fixture
    Capture(CaptureArgs),

    /// Capture all combinations for a provider (or all providers if --provider is omitted)
    CaptureAll(CaptureAllArgs),

    /// Manifest operations
    #[command(subcommand)]
    Manifest(ManifestCommands),
}

#[derive(clap::Args)]
struct CaptureArgs {
    /// Provider slug (e.g. "kitty")
    #[arg(long)]
    provider: String,

    /// Theme name or slug (e.g. "tokyo-night" or "Tokyo Night")
    #[arg(long)]
    theme: String,

    /// Fixture id (e.g. "git-diff")
    #[arg(long)]
    fixture: String,

    /// Directory to write output images
    #[arg(long, default_value = "./staging")]
    staging_dir: PathBuf,

    /// Directory containing fixture subdirectories
    #[arg(long, default_value = "./fixtures")]
    fixtures_dir: PathBuf,

    /// Directory containing theme .toml files
    #[arg(long, default_value = "./themes")]
    themes_dir: PathBuf,

    /// Seconds to wait for fixture command to complete
    #[arg(long, default_value_t = 30)]
    timeout: u64,
}

#[derive(clap::Args)]
struct CaptureAllArgs {
    /// Provider slug; if omitted, captures all registered providers
    #[arg(long)]
    provider: Option<String>,

    /// Directory to write output images
    #[arg(long, default_value = "./staging")]
    staging_dir: PathBuf,

    /// Directory containing fixture subdirectories
    #[arg(long, default_value = "./fixtures")]
    fixtures_dir: PathBuf,

    /// Directory containing theme .toml files
    #[arg(long, default_value = "./themes")]
    themes_dir: PathBuf,

    /// Seconds to wait per fixture command
    #[arg(long, default_value_t = 30)]
    timeout: u64,

    /// Continue on failure (don't abort on first error)
    #[arg(long, default_value_t = false)]
    keep_going: bool,
}

#[derive(Subcommand)]
enum ManifestCommands {
    /// Build manifest.json by scanning the staging directory
    Build(ManifestBuildArgs),

    /// Check coverage: how many (provider, theme, fixture) combos are present
    Check(ManifestCheckArgs),
}

#[derive(clap::Args)]
struct ManifestBuildArgs {
    /// Staging directory to scan
    #[arg(long, default_value = "./staging")]
    staging_dir: PathBuf,

    /// Base URL for the CDN (written into manifest.json)
    #[arg(long, env = "LITMUS_SCREENSHOTS_BASE_URL", default_value = "https://screenshots.litmus.edger.dev")]
    base_url: String,

    /// Output path for manifest.json
    #[arg(long, default_value = "./staging/manifest.json")]
    output: PathBuf,

    /// Directory containing fixture subdirectories (for fixture metadata)
    #[arg(long, default_value = "./fixtures")]
    fixtures_dir: PathBuf,
}

#[derive(clap::Args)]
struct ManifestCheckArgs {
    /// Path to manifest.json
    #[arg(long, default_value = "./staging/manifest.json")]
    manifest: PathBuf,

    /// Directory containing theme .toml files (to determine expected themes)
    #[arg(long, default_value = "./themes")]
    themes_dir: PathBuf,

    /// Directory containing fixture subdirectories (to determine expected fixtures)
    #[arg(long, default_value = "./fixtures")]
    fixtures_dir: PathBuf,

    /// Provider slugs to check coverage for (repeatable; default: all registered)
    #[arg(long)]
    provider: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Capture(args) => cmd_capture(args),
        Commands::CaptureAll(args) => cmd_capture_all(args),
        Commands::Manifest(ManifestCommands::Build(args)) => cmd_manifest_build(args),
        Commands::Manifest(ManifestCommands::Check(args)) => cmd_manifest_check(args),
    }
}

fn cmd_capture(args: CaptureArgs) -> Result<()> {
    let provider = find_provider(&args.provider)
        .with_context(|| format!("unknown provider '{}'; available: {}", args.provider, available_providers()))?;

    let theme = load_theme_by_slug(&args.themes_dir, &args.theme)
        .with_context(|| format!("theme '{}' not found in {}", args.theme, args.themes_dir.display()))?;

    let fixture_dir = args.fixtures_dir.join(&args.fixture);
    if !fixture_dir.exists() {
        bail!("fixture '{}' not found at {}", args.fixture, fixture_dir.display());
    }

    let theme_slug = theme_to_slug(&theme.name);
    let output_path = args
        .staging_dir
        .join(&args.provider)
        .join(&theme_slug)
        .join(format!("{}.webp", args.fixture));

    eprintln!(
        "Capturing: provider={} theme={} fixture={} → {}",
        args.provider,
        theme_slug,
        args.fixture,
        output_path.display()
    );

    let result = capture_screenshot(&CaptureOptions {
        provider: provider.as_ref(),
        theme: &theme,
        fixture_dir: &fixture_dir,
        fixture_id: &args.fixture,
        output_path: &output_path,
        geometry: TermGeometry::default(),
        timeout_secs: args.timeout,
    })?;

    eprintln!(
        "  ✓ {}x{} checksum={}",
        result.width,
        result.height,
        &result.checksum[..16]
    );

    Ok(())
}

fn cmd_capture_all(args: CaptureAllArgs) -> Result<()> {
    let providers: Vec<Box<dyn providers::ProviderCapture>> = match &args.provider {
        Some(slug) => {
            let p = find_provider(slug)
                .with_context(|| format!("unknown provider '{}'; available: {}", slug, available_providers()))?;
            vec![p]
        }
        None => all_providers(),
    };

    let themes = load_all_themes(&args.themes_dir)?;
    let fixture_ids = list_fixture_ids(&args.fixtures_dir)?;

    let total = providers.len() * themes.len() * fixture_ids.len();
    let mut done = 0;
    let mut failed = 0;

    for provider in &providers {
        for theme in &themes {
            let theme_slug = theme_to_slug(&theme.name);
            for fixture_id in &fixture_ids {
                let fixture_dir = args.fixtures_dir.join(fixture_id);
                let output_path = args
                    .staging_dir
                    .join(provider.slug())
                    .join(&theme_slug)
                    .join(format!("{}.webp", fixture_id));

                eprintln!(
                    "[{}/{}] {} / {} / {}",
                    done + 1,
                    total,
                    provider.slug(),
                    theme_slug,
                    fixture_id
                );

                match capture_screenshot(&CaptureOptions {
                    provider: provider.as_ref(),
                    theme,
                    fixture_dir: &fixture_dir,
                    fixture_id,
                    output_path: &output_path,
                    geometry: TermGeometry::default(),
                    timeout_secs: args.timeout,
                }) {
                    Ok(result) => {
                        eprintln!("  ✓ {}x{}", result.width, result.height);
                    }
                    Err(e) => {
                        eprintln!("  ✗ Error: {:#}", e);
                        failed += 1;
                        if !args.keep_going {
                            bail!("capture failed (use --keep-going to continue on errors)");
                        }
                    }
                }

                done += 1;
            }
        }
    }

    eprintln!("\nDone: {}/{} succeeded, {} failed", done - failed, total, failed);
    if failed > 0 {
        bail!("{} captures failed", failed);
    }
    Ok(())
}

fn cmd_manifest_build(args: ManifestBuildArgs) -> Result<()> {
    let providers = all_providers()
        .into_iter()
        .map(|p| Provider {
            slug: p.slug().to_string(),
            name: p.name().to_string(),
            version: None,
        })
        .collect();

    let fixtures = list_fixture_ids(&args.fixtures_dir)?
        .into_iter()
        .map(|id| Fixture {
            name: fixture_id_to_name(&id),
            description: String::new(),
            id,
        })
        .collect();

    let manifest = build_manifest_from_staging(
        &args.staging_dir,
        &args.base_url,
        providers,
        fixtures,
    )?;

    let json = serde_json::to_string_pretty(&manifest).context("serialize manifest")?;

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent).context("create output directory")?;
    }
    fs::write(&args.output, &json).context("write manifest.json")?;

    eprintln!(
        "Manifest written to {} ({} screenshots, {} providers)",
        args.output.display(),
        manifest.screenshots.len(),
        manifest.providers.len()
    );

    Ok(())
}

fn cmd_manifest_check(args: ManifestCheckArgs) -> Result<()> {
    let json = fs::read_to_string(&args.manifest)
        .with_context(|| format!("read {}", args.manifest.display()))?;
    let manifest: ScreenshotManifest =
        serde_json::from_str(&json).context("parse manifest.json")?;

    let themes = load_all_themes(&args.themes_dir)?;
    let theme_slugs: Vec<String> = themes.iter().map(|t| theme_to_slug(&t.name)).collect();

    let fixture_ids = list_fixture_ids(&args.fixtures_dir)?;

    let provider_slugs: Vec<String> = if args.provider.is_empty() {
        all_providers().iter().map(|p| p.slug().to_string()).collect()
    } else {
        args.provider.clone()
    };

    let report = CoverageReport::check(
        &manifest,
        &provider_slugs.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        &theme_slugs.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        &fixture_ids.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    );

    println!(
        "Coverage: {}/{} ({:.1}%)",
        report.total_present,
        report.total_expected,
        report.coverage_pct()
    );

    if !report.is_complete() {
        println!("\nMissing ({}):", report.missing.len());
        for (provider, theme, fixture) in &report.missing {
            println!("  {provider}/{theme}/{fixture}");
        }
    } else {
        println!("All combinations present ✓");
    }

    if !report.is_complete() {
        std::process::exit(1);
    }

    Ok(())
}

// --- Helpers ---

fn available_providers() -> String {
    all_providers()
        .iter()
        .map(|p| p.slug())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Load a theme by exact name or slug from the themes directory.
fn load_theme_by_slug(themes_dir: &Path, query: &str) -> Result<Theme> {
    let themes = load_all_themes(themes_dir)?;
    let query_slug = theme_to_slug(query);

    themes
        .into_iter()
        .find(|t| {
            theme_to_slug(&t.name) == query_slug || t.name.to_lowercase() == query.to_lowercase()
        })
        .context("theme not found")
}

/// Load all themes from a directory of .toml, .conf, and .yaml files.
pub fn load_all_themes(themes_dir: &Path) -> Result<Vec<Theme>> {
    let mut themes = Vec::new();

    let entries = fs::read_dir(themes_dir)
        .with_context(|| format!("read themes dir {}", themes_dir.display()))?;

    for entry in entries {
        let entry = entry.context("read directory entry")?;
        let path = entry.path();

        let theme = match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("read {}", path.display()))?;
                parse_toml_theme(&content)
                    .with_context(|| format!("parse {}", path.display()))?
            }
            Some("conf") => {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("read {}", path.display()))?;
                parse_kitty_theme(&content)
                    .with_context(|| format!("parse {}", path.display()))?
            }
            _ => continue,
        };

        themes.push(theme);
    }

    themes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(themes)
}

/// Convert a theme name to a URL/filesystem slug.
/// "Tokyo Night" → "tokyo-night"
pub fn theme_to_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// List fixture IDs (subdirectory names) in the fixtures directory.
fn list_fixture_ids(fixtures_dir: &Path) -> Result<Vec<String>> {
    let mut ids = Vec::new();

    let entries = fs::read_dir(fixtures_dir)
        .with_context(|| format!("read fixtures dir {}", fixtures_dir.display()))?;

    for entry in entries {
        let entry = entry.context("read directory entry")?;
        let path = entry.path();
        if path.is_dir() {
            // Only include if command.sh exists
            if path.join("command.sh").exists() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    ids.push(name.to_string());
                }
            }
        }
    }

    ids.sort();
    Ok(ids)
}

/// Convert fixture id to display name: "git-diff" → "Git Diff"
fn fixture_id_to_name(id: &str) -> String {
    id.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_to_slug_simple() {
        assert_eq!(theme_to_slug("Tokyo Night"), "tokyo-night");
    }

    #[test]
    fn theme_to_slug_special_chars() {
        assert_eq!(theme_to_slug("Catppuccin Mocha"), "catppuccin-mocha");
        assert_eq!(theme_to_slug("Nord"), "nord");
        assert_eq!(theme_to_slug("One Dark Pro"), "one-dark-pro");
    }

    #[test]
    fn theme_to_slug_no_double_hyphens() {
        assert_eq!(theme_to_slug("Foo  Bar"), "foo-bar");
        assert_eq!(theme_to_slug("Foo - Bar"), "foo-bar");
    }

    #[test]
    fn fixture_id_to_name_basic() {
        assert_eq!(fixture_id_to_name("git-diff"), "Git Diff");
        assert_eq!(fixture_id_to_name("ls-color"), "Ls Color");
        assert_eq!(fixture_id_to_name("cargo-build"), "Cargo Build");
    }
}
