use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A terminal emulator or app that provides color definitions.
/// Examples: kitty, wezterm, alacritty. Later: neovim (as a silo).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    /// Machine-readable identifier, e.g. "kitty"
    pub slug: String,
    /// Human-readable display name, e.g. "Kitty"
    pub name: String,
    /// Optional version string recorded at capture time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// A reproducible terminal scenario captured from real commands.
/// Each fixture corresponds to a `fixtures/{id}/` directory in the repo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    /// Machine-readable identifier matching the fixtures/ directory name, e.g. "git-diff"
    pub id: String,
    /// Human-readable display name, e.g. "Git Diff"
    pub name: String,
    /// What this fixture demonstrates
    pub description: String,
}

/// The composite key identifying one screenshot: a (provider, theme, fixture) triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScreenshotKey {
    /// Provider slug, e.g. "kitty"
    pub provider: String,
    /// Theme slug, e.g. "tokyo-night"
    pub theme: String,
    /// Fixture id, e.g. "git-diff"
    pub fixture: String,
}

/// The image format used for a screenshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Webp,
}

impl ImageFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Webp => "webp",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Webp => "image/webp",
        }
    }
}

/// Metadata for a single captured screenshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotMeta {
    #[serde(flatten)]
    pub key: ScreenshotKey,
    /// Path relative to the manifest's `base_url`, e.g. "v1/kitty/tokyo-night/git-diff.webp"
    pub url: String,
    /// Pixel width (at full/2x resolution)
    pub width: u32,
    /// Pixel height (at full/2x resolution)
    pub height: u32,
    pub format: ImageFormat,
    /// ISO 8601 timestamp of when this screenshot was captured
    pub captured_at: String,
    /// SHA-256 hex digest of the image file, used for cache busting
    pub checksum: String,
}

impl ScreenshotMeta {
    /// Returns the full URL for this screenshot given the manifest's base URL.
    pub fn full_url(&self, base_url: &str) -> String {
        let base = base_url.trim_end_matches('/');
        format!("{}/{}", base, self.url.trim_start_matches('/'))
    }

    /// Returns a cache-busted URL using the first 8 chars of the checksum.
    pub fn cache_busted_url(&self, base_url: &str) -> String {
        format!("{}?v={}", self.full_url(base_url), &self.checksum[..8.min(self.checksum.len())])
    }
}

/// The top-level index of all captured screenshots.
/// Serialized as `manifest.json` at the root of the CDN bucket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotManifest {
    /// Schema version for forward compatibility
    pub version: u32,
    /// Base URL of the CDN bucket, e.g. "https://screenshots.litmus.edger.dev"
    pub base_url: String,
    /// All known providers that have been captured
    pub providers: Vec<Provider>,
    /// All known fixtures that have been captured
    pub fixtures: Vec<Fixture>,
    /// All captured screenshots
    pub screenshots: Vec<ScreenshotMeta>,
}

impl ScreenshotManifest {
    /// Build a lookup index for O(1) screenshot retrieval by key.
    /// Returns a map from (provider, theme, fixture) to screenshot index.
    pub fn build_index(&self) -> HashMap<ScreenshotKey, usize> {
        self.screenshots
            .iter()
            .enumerate()
            .map(|(i, s)| (s.key.clone(), i))
            .collect()
    }

    /// Find a screenshot by provider slug, theme slug, and fixture id.
    pub fn find(&self, provider: &str, theme: &str, fixture: &str) -> Option<&ScreenshotMeta> {
        self.screenshots.iter().find(|s| {
            s.key.provider == provider && s.key.theme == theme && s.key.fixture == fixture
        })
    }

    /// Returns all screenshots for a given provider slug.
    pub fn for_provider(&self, provider: &str) -> Vec<&ScreenshotMeta> {
        self.screenshots
            .iter()
            .filter(|s| s.key.provider == provider)
            .collect()
    }

    /// Returns all screenshots for a given theme slug.
    pub fn for_theme(&self, theme: &str) -> Vec<&ScreenshotMeta> {
        self.screenshots
            .iter()
            .filter(|s| s.key.theme == theme)
            .collect()
    }

    /// Returns provider metadata by slug.
    pub fn provider(&self, slug: &str) -> Option<&Provider> {
        self.providers.iter().find(|p| p.slug == slug)
    }

    /// Returns fixture metadata by id.
    pub fn fixture(&self, id: &str) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == id)
    }

    /// Returns all distinct provider slugs that have at least one screenshot.
    pub fn active_provider_slugs(&self) -> Vec<&str> {
        let mut slugs: Vec<&str> = self
            .screenshots
            .iter()
            .map(|s| s.key.provider.as_str())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        slugs.sort();
        slugs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> ScreenshotManifest {
        ScreenshotManifest {
            version: 1,
            base_url: "https://screenshots.litmus.edger.dev".to_string(),
            providers: vec![
                Provider {
                    slug: "kitty".to_string(),
                    name: "Kitty".to_string(),
                    version: Some("0.35.0".to_string()),
                },
                Provider {
                    slug: "wezterm".to_string(),
                    name: "WezTerm".to_string(),
                    version: None,
                },
            ],
            fixtures: vec![
                Fixture {
                    id: "git-diff".to_string(),
                    name: "Git Diff".to_string(),
                    description: "Git diff with additions, deletions, and context lines".to_string(),
                },
                Fixture {
                    id: "ls-color".to_string(),
                    name: "Directory Listing".to_string(),
                    description: "ls -la output with colorized file types".to_string(),
                },
            ],
            screenshots: vec![
                ScreenshotMeta {
                    key: ScreenshotKey {
                        provider: "kitty".to_string(),
                        theme: "tokyo-night".to_string(),
                        fixture: "git-diff".to_string(),
                    },
                    url: "v1/kitty/tokyo-night/git-diff.webp".to_string(),
                    width: 1600,
                    height: 1000,
                    format: ImageFormat::Webp,
                    captured_at: "2026-03-23T00:00:00Z".to_string(),
                    checksum: "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
                },
                ScreenshotMeta {
                    key: ScreenshotKey {
                        provider: "kitty".to_string(),
                        theme: "catppuccin-mocha".to_string(),
                        fixture: "git-diff".to_string(),
                    },
                    url: "v1/kitty/catppuccin-mocha/git-diff.webp".to_string(),
                    width: 1600,
                    height: 1000,
                    format: ImageFormat::Webp,
                    captured_at: "2026-03-23T00:00:00Z".to_string(),
                    checksum: "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
                },
            ],
        }
    }

    #[test]
    fn serde_round_trip_json() {
        let manifest = sample_manifest();
        let json = serde_json::to_string(&manifest).expect("serialize");
        let parsed: ScreenshotManifest = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(manifest, parsed);
    }

    #[test]
    fn serde_round_trip_pretty_json() {
        let manifest = sample_manifest();
        let json = serde_json::to_string_pretty(&manifest).expect("serialize");
        let parsed: ScreenshotManifest = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(manifest, parsed);
    }

    #[test]
    fn find_screenshot() {
        let manifest = sample_manifest();
        let s = manifest.find("kitty", "tokyo-night", "git-diff");
        assert!(s.is_some());
        assert_eq!(s.unwrap().url, "v1/kitty/tokyo-night/git-diff.webp");
    }

    #[test]
    fn find_missing_screenshot() {
        let manifest = sample_manifest();
        assert!(manifest.find("kitty", "tokyo-night", "htop").is_none());
        assert!(manifest.find("alacritty", "tokyo-night", "git-diff").is_none());
    }

    #[test]
    fn for_provider() {
        let manifest = sample_manifest();
        let kitty = manifest.for_provider("kitty");
        assert_eq!(kitty.len(), 2);
        let wezterm = manifest.for_provider("wezterm");
        assert_eq!(wezterm.len(), 0);
    }

    #[test]
    fn for_theme() {
        let manifest = sample_manifest();
        let tn = manifest.for_theme("tokyo-night");
        assert_eq!(tn.len(), 1);
    }

    #[test]
    fn full_url() {
        let manifest = sample_manifest();
        let s = manifest.find("kitty", "tokyo-night", "git-diff").unwrap();
        assert_eq!(
            s.full_url("https://screenshots.litmus.edger.dev"),
            "https://screenshots.litmus.edger.dev/v1/kitty/tokyo-night/git-diff.webp"
        );
    }

    #[test]
    fn full_url_trailing_slash() {
        let manifest = sample_manifest();
        let s = manifest.find("kitty", "tokyo-night", "git-diff").unwrap();
        assert_eq!(
            s.full_url("https://screenshots.litmus.edger.dev/"),
            "https://screenshots.litmus.edger.dev/v1/kitty/tokyo-night/git-diff.webp"
        );
    }

    #[test]
    fn cache_busted_url() {
        let manifest = sample_manifest();
        let s = manifest.find("kitty", "tokyo-night", "git-diff").unwrap();
        let url = s.cache_busted_url("https://screenshots.litmus.edger.dev");
        assert!(url.contains("?v=abcdef12"));
    }

    #[test]
    fn build_index() {
        let manifest = sample_manifest();
        let index = manifest.build_index();
        assert_eq!(index.len(), 2);
        let key = ScreenshotKey {
            provider: "kitty".to_string(),
            theme: "tokyo-night".to_string(),
            fixture: "git-diff".to_string(),
        };
        assert!(index.contains_key(&key));
    }

    #[test]
    fn active_provider_slugs() {
        let manifest = sample_manifest();
        let slugs = manifest.active_provider_slugs();
        assert_eq!(slugs, vec!["kitty"]);
    }

    #[test]
    fn provider_lookup() {
        let manifest = sample_manifest();
        let kitty = manifest.provider("kitty").unwrap();
        assert_eq!(kitty.name, "Kitty");
        assert_eq!(kitty.version.as_deref(), Some("0.35.0"));

        let wez = manifest.provider("wezterm").unwrap();
        assert!(wez.version.is_none());

        assert!(manifest.provider("alacritty").is_none());
    }

    #[test]
    fn fixture_lookup() {
        let manifest = sample_manifest();
        let f = manifest.fixture("git-diff").unwrap();
        assert_eq!(f.name, "Git Diff");
        assert!(manifest.fixture("htop").is_none());
    }

    #[test]
    fn image_format_extension() {
        assert_eq!(ImageFormat::Webp.extension(), "webp");
        assert_eq!(ImageFormat::Png.extension(), "png");
    }

    #[test]
    fn image_format_mime_type() {
        assert_eq!(ImageFormat::Webp.mime_type(), "image/webp");
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
    }

    #[test]
    fn json_shape_flattened_key() {
        // Verify that ScreenshotKey fields are flattened into the screenshot object
        let manifest = sample_manifest();
        let json = serde_json::to_value(&manifest).unwrap();
        let first = &json["screenshots"][0];
        assert_eq!(first["provider"], "kitty");
        assert_eq!(first["theme"], "tokyo-night");
        assert_eq!(first["fixture"], "git-diff");
        // These should NOT be nested under a "key" object
        assert!(first.get("key").is_none());
    }

    #[test]
    fn version_field_omitted_when_none() {
        let provider = Provider {
            slug: "alacritty".to_string(),
            name: "Alacritty".to_string(),
            version: None,
        };
        let json = serde_json::to_string(&provider).unwrap();
        assert!(!json.contains("version"));
    }
}
