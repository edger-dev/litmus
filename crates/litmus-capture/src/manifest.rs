use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use walkdir::WalkDir;

use litmus_model::screenshot::{
    Fixture, ImageFormat, Provider, ScreenshotManifest, ScreenshotMeta,
    ScreenshotKey,
};

use crate::capture::sha256_hex;

/// Build a `ScreenshotManifest` by scanning the staging directory.
///
/// Expected layout:
/// ```
/// staging/
///   {provider}/
///     {theme}/
///       {fixture}.webp    (or .png)
/// ```
///
/// Provider and fixture metadata must be supplied separately (via registered
/// providers and fixture directories) since the staging directory only has slugs.
pub fn build_manifest_from_staging(
    staging_dir: &Path,
    base_url: &str,
    providers: Vec<Provider>,
    fixtures: Vec<Fixture>,
) -> Result<ScreenshotManifest> {
    let mut screenshots = Vec::new();

    for entry in WalkDir::new(staging_dir)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        // Extract format from extension
        let format = match path.extension().and_then(|e| e.to_str()) {
            Some("webp") => ImageFormat::Webp,
            Some("png") => ImageFormat::Png,
            _ => continue, // Skip non-image files
        };

        // Parse path components: staging/{provider}/{theme}/{fixture}.ext
        let mut components = path
            .strip_prefix(staging_dir)
            .context("strip staging prefix")?
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned());

        let provider_slug = components.next().context("provider component")?;
        let theme_slug = components.next().context("theme component")?;
        let filename = components.next().context("filename component")?;
        let fixture_id = Path::new(&filename)
            .file_stem()
            .context("fixture stem")?
            .to_string_lossy()
            .into_owned();

        // Read the image to get dimensions and checksum
        let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
        let (width, height) = {
            let reader = image::ImageReader::open(path)
                .with_context(|| format!("open image {}", path.display()))?;
            reader.into_dimensions().context("read dimensions")?
        };
        let checksum = sha256_hex(&bytes);

        // Build the relative URL (relative to base_url)
        let url = format!("v1/{}/{}/{}.{}", provider_slug, theme_slug, fixture_id, format.extension());

        let captured_at = Utc::now().to_rfc3339();

        screenshots.push(ScreenshotMeta {
            key: ScreenshotKey {
                provider: provider_slug,
                theme: theme_slug,
                fixture: fixture_id,
            },
            url,
            width,
            height,
            format,
            captured_at,
            checksum,
        });
    }

    // Sort for deterministic output
    screenshots.sort_by(|a, b| {
        a.key
            .provider
            .cmp(&b.key.provider)
            .then(a.key.theme.cmp(&b.key.theme))
            .then(a.key.fixture.cmp(&b.key.fixture))
    });

    Ok(ScreenshotManifest {
        version: 1,
        base_url: base_url.trim_end_matches('/').to_string(),
        providers,
        fixtures,
        screenshots,
    })
}

/// Print a coverage report: which (provider, theme, fixture) combinations are present
/// in the manifest vs. expected.
pub struct CoverageReport {
    pub total_expected: usize,
    pub total_present: usize,
    pub missing: Vec<(String, String, String)>,
}

impl CoverageReport {
    /// Check coverage: how many of the expected combinations are in the manifest.
    pub fn check(
        manifest: &ScreenshotManifest,
        provider_slugs: &[&str],
        theme_slugs: &[&str],
        fixture_ids: &[&str],
    ) -> Self {
        let total_expected = provider_slugs.len() * theme_slugs.len() * fixture_ids.len();
        let mut missing = Vec::new();

        for &provider in provider_slugs {
            for &theme in theme_slugs {
                for &fixture in fixture_ids {
                    if manifest.find(provider, theme, fixture).is_none() {
                        missing.push((provider.to_string(), theme.to_string(), fixture.to_string()));
                    }
                }
            }
        }

        let total_present = total_expected - missing.len();
        CoverageReport { total_expected, total_present, missing }
    }

    pub fn is_complete(&self) -> bool {
        self.missing.is_empty()
    }

    pub fn coverage_pct(&self) -> f64 {
        if self.total_expected == 0 {
            return 100.0;
        }
        self.total_present as f64 / self.total_expected as f64 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use litmus_model::screenshot::{Provider, Fixture};

    fn make_manifest() -> ScreenshotManifest {
        ScreenshotManifest {
            version: 1,
            base_url: "https://example.com".to_string(),
            providers: vec![Provider { slug: "kitty".to_string(), name: "Kitty".to_string(), version: None }],
            fixtures: vec![Fixture { id: "git-diff".to_string(), name: "Git Diff".to_string(), description: "".to_string() }],
            screenshots: vec![ScreenshotMeta {
                key: ScreenshotKey {
                    provider: "kitty".to_string(),
                    theme: "tokyo-night".to_string(),
                    fixture: "git-diff".to_string(),
                },
                url: "v1/kitty/tokyo-night/git-diff.webp".to_string(),
                width: 1600,
                height: 1000,
                format: ImageFormat::Webp,
                captured_at: "2026-01-01T00:00:00Z".to_string(),
                checksum: "abc123".to_string(),
            }],
        }
    }

    #[test]
    fn coverage_complete() {
        let manifest = make_manifest();
        let report = CoverageReport::check(
            &manifest,
            &["kitty"],
            &["tokyo-night"],
            &["git-diff"],
        );
        assert!(report.is_complete());
        assert_eq!(report.total_expected, 1);
        assert_eq!(report.total_present, 1);
        assert!((report.coverage_pct() - 100.0).abs() < 0.01);
    }

    #[test]
    fn coverage_with_missing() {
        let manifest = make_manifest();
        let report = CoverageReport::check(
            &manifest,
            &["kitty"],
            &["tokyo-night", "catppuccin-mocha"],
            &["git-diff"],
        );
        assert!(!report.is_complete());
        assert_eq!(report.total_expected, 2);
        assert_eq!(report.total_present, 1);
        assert_eq!(report.missing.len(), 1);
        assert_eq!(report.missing[0], ("kitty".to_string(), "catppuccin-mocha".to_string(), "git-diff".to_string()));
    }

    #[test]
    fn coverage_empty_manifest() {
        let mut manifest = make_manifest();
        manifest.screenshots.clear();
        let report = CoverageReport::check(
            &manifest,
            &["kitty"],
            &["tokyo-night"],
            &["git-diff"],
        );
        assert!(!report.is_complete());
        assert_eq!(report.total_present, 0);
    }
}
