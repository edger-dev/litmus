mod ansi_parser;
mod capture;
mod error;
mod extract;
mod manifest;
mod providers;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use litmus_model::screenshot::{Fixture, Provider, ScreenshotManifest};
use litmus_model::Theme;

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

    /// Extract provider colors from vendored theme data into per-provider TOML files
    ExtractColors(ExtractColorsArgs),

    /// Parse raw ANSI output into structured TermOutput JSON
    ParseAnsi(ParseAnsiArgs),

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

    /// Number of parallel capture workers (default: number of CPU cores)
    #[arg(long, short = 'j')]
    jobs: Option<usize>,

    /// Re-capture even if output file already exists
    #[arg(long, default_value_t = false)]
    force: bool,
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

    /// URL path prefix for screenshot paths (e.g. "v1" → "v1/foot/theme/fixture.webp").
    /// Use empty string for local dev where staging dir is served directly.
    #[arg(long, default_value = "v1")]
    url_prefix: String,

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

#[derive(clap::Args)]
struct ParseAnsiArgs {
    /// Input file containing raw ANSI output (default: stdin)
    #[arg(long)]
    input: Option<PathBuf>,

    /// Fixture ID for the output metadata
    #[arg(long, default_value = "unknown")]
    id: String,

    /// Display name for the output metadata
    #[arg(long, default_value = "Unknown")]
    name: String,

    /// Terminal width in columns
    #[arg(long, default_value_t = 80)]
    cols: u16,

    /// Terminal height in rows
    #[arg(long, default_value_t = 24)]
    rows: u16,

    /// Output file (default: stdout)
    #[arg(long, short)]
    output: Option<PathBuf>,
}

#[derive(clap::Args)]
struct ExtractColorsArgs {
    /// Directory containing ThemeDefinition .toml files
    #[arg(long, default_value = "./themes")]
    themes_dir: PathBuf,

    /// Directory containing vendored provider theme data
    #[arg(long, default_value = "./vendor")]
    vendor_dir: PathBuf,

    /// Filter to a single provider (e.g. "kitty" or "wezterm")
    #[arg(long)]
    provider: Option<String>,

    /// Filter to a single theme slug (e.g. "gruvbox-dark")
    #[arg(long)]
    theme: Option<String>,

    /// Overwrite existing generated files
    #[arg(long, default_value_t = false)]
    force: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Capture(args) => cmd_capture(args),
        Commands::CaptureAll(args) => cmd_capture_all(args),
        Commands::ExtractColors(args) => cmd_extract_colors(args),
        Commands::ParseAnsi(args) => cmd_parse_ansi(args),
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
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    // Build work items: (provider_idx, theme_idx, fixture_id)
    let mut work: Vec<(usize, usize, String)> = Vec::new();
    for (pi, _) in providers.iter().enumerate() {
        for (ti, _) in themes.iter().enumerate() {
            for fid in &fixture_ids {
                work.push((pi, ti, fid.clone()));
            }
        }
    }

    let total = work.len();
    let num_jobs = args.jobs.unwrap_or_else(num_cpus::get);
    eprintln!("Capturing {} screenshots with {} workers...", total, num_jobs);

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_jobs)
        .build_global()
        .ok(); // Ignore error if pool already initialized

    let done = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);
    let failed = AtomicUsize::new(0);
    let force = args.force;

    let results: Vec<Option<anyhow::Error>> = work
        .par_iter()
        .map(|(pi, ti, fixture_id)| {
            let provider = &providers[*pi];
            let theme = &themes[*ti];
            let theme_slug = theme_to_slug(&theme.name);
            let fixture_dir = args.fixtures_dir.join(fixture_id);
            let output_path = args
                .staging_dir
                .join(provider.slug())
                .join(&theme_slug)
                .join(format!("{}.webp", fixture_id));

            let n = done.fetch_add(1, Ordering::Relaxed) + 1;

            if !force && output_path.exists() {
                skipped.fetch_add(1, Ordering::Relaxed);
                eprintln!(
                    "[{}/{}] {} / {} / {} (skipped, exists)",
                    n, total, provider.slug(), theme_slug, fixture_id
                );
                return None;
            }

            eprintln!(
                "[{}/{}] {} / {} / {}",
                n, total, provider.slug(), theme_slug, fixture_id
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
                    eprintln!("  ✓ {} / {} / {} {}x{}", provider.slug(), theme_slug, fixture_id, result.width, result.height);
                    None
                }
                Err(e) => {
                    eprintln!("  ✗ {} / {} / {} Error: {:#}", provider.slug(), theme_slug, fixture_id, e);
                    failed.fetch_add(1, Ordering::Relaxed);
                    Some(e)
                }
            }
        })
        .collect();

    let failed_count = failed.load(Ordering::Relaxed);
    let skipped_count = skipped.load(Ordering::Relaxed);
    let captured = total - failed_count - skipped_count;
    eprintln!(
        "\nDone: {} captured, {} skipped, {} failed (of {} total)",
        captured, skipped_count, failed_count, total
    );

    if !args.keep_going
        && let Some(err) = results.into_iter().flatten().next()
    {
        return Err(err.context("capture failed (use --keep-going to continue on errors)"));
    }

    if failed_count > 0 {
        bail!("{} captures failed", failed_count);
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
        &args.url_prefix,
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

fn cmd_parse_ansi(args: ParseAnsiArgs) -> Result<()> {
    use std::io::Read;

    let input = match &args.input {
        Some(path) => fs::read(path).with_context(|| format!("read {}", path.display()))?,
        None => {
            let mut buf = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buf)
                .context("read stdin")?;
            buf
        }
    };

    let output = ansi_parser::parse_ansi(&input, args.cols, args.rows, &args.id, &args.name);
    let json = serde_json::to_string_pretty(&output).context("serialize TermOutput")?;

    match &args.output {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, &json).with_context(|| format!("write {}", path.display()))?;
            eprintln!("Written to {}", path.display());
        }
        None => println!("{json}"),
    }

    Ok(())
}

fn cmd_extract_colors(args: ExtractColorsArgs) -> Result<()> {
    use crate::extract::{
        build_kitty_index, build_wezterm_index, extract_provider_colors, find_theme_definitions,
    };

    eprintln!("Building vendor indexes...");
    let kitty_index = build_kitty_index(&args.vendor_dir)?;
    let wezterm_index = build_wezterm_index(&args.vendor_dir)?;
    eprintln!(
        "  kitty: {} themes, wezterm: {} themes",
        kitty_index.len(),
        wezterm_index.len()
    );

    let definitions = find_theme_definitions(&args.themes_dir)?;
    eprintln!("Found {} theme definitions", definitions.len());

    if definitions.is_empty() {
        eprintln!("No ThemeDefinition files found. Theme files must have a [providers] section.");
        return Ok(());
    }

    let mut extracted = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for (def, parent_dir) in &definitions {
        if let Some(ref filter) = args.theme
            && def.slug != *filter
        {
            continue;
        }

        for (provider_slug, provider_theme_name) in &def.providers {
            if let Some(ref filter) = args.provider
                && provider_slug != filter
            {
                continue;
            }

            let output_path = parent_dir.join(format!("{}.{}.toml", def.slug, provider_slug));

            if !args.force && output_path.exists() {
                skipped += 1;
                continue;
            }

            match extract_provider_colors(
                &args.vendor_dir,
                provider_slug,
                provider_theme_name,
                &kitty_index,
                &wezterm_index,
            ) {
                Ok(colors) => {
                    let toml_content = colors.to_toml();
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&output_path, toml_content)
                        .with_context(|| format!("write {}", output_path.display()))?;
                    eprintln!(
                        "  {} → {} ({})",
                        def.slug,
                        provider_slug,
                        output_path.display()
                    );
                    extracted += 1;
                }
                Err(e) => {
                    eprintln!(
                        "  ✗ {} → {} error: {:#}",
                        def.slug, provider_slug, e
                    );
                    failed += 1;
                }
            }
        }
    }

    eprintln!(
        "\nDone: {} extracted, {} skipped (exist), {} failed",
        extracted, skipped, failed
    );

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

/// Load all themes from a directory of ThemeDefinition + ProviderColors files.
/// Returns one Theme per definition using the first available provider's colors.
pub fn load_all_themes(themes_dir: &Path) -> Result<Vec<Theme>> {
    let (definitions, colors) = litmus_model::provider::load_themes_dir(themes_dir)
        .with_context(|| format!("load themes from {}", themes_dir.display()))?;

    let mut themes = Vec::new();
    for def in &definitions {
        // Pick first available provider (sorted for determinism)
        let mut providers: Vec<&String> = def.providers.keys().collect();
        providers.sort();
        if let Some(pc) = providers
            .into_iter()
            .find_map(|p| colors.get(&(def.slug.clone(), p.clone())))
        {
            themes.push(pc.to_theme(&def.name));
        }
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
            if path.join("command.sh").exists()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
            {
                ids.push(name.to_string());
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
