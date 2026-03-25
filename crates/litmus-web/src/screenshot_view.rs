use dioxus::prelude::*;
use litmus_model::screenshot::ScreenshotManifest;

use crate::state::ManifestState;

/// Display a real terminal screenshot image.
///
/// If provider is specified, looks up that provider. Otherwise auto-picks first available.
/// Returns an empty element if no screenshot is found.
#[component]
pub fn ScreenshotImage(
    /// Theme slug (e.g. "tokyo-night")
    theme_slug: String,
    /// Fixture id (e.g. "git-diff")
    fixture_id: String,
    /// Provider slug (e.g. "kitty"). If empty, auto-picks first available.
    #[props(default = String::new())]
    provider: String,
) -> Element {
    let manifest_state = use_context::<Signal<ManifestState>>();
    let manifest = manifest_state.read();

    let result = if provider.is_empty() {
        find_any_screenshot_url(&manifest.0, &theme_slug, &fixture_id)
    } else {
        find_screenshot_url(&manifest.0, &provider, &theme_slug, &fixture_id)
    };

    match result {
        Some((url, width, height)) => {
            let display_width = width / 2; // 2x capture → display at 1x
            let display_height = height / 2;

            rsx! {
                img {
                    src: "{url}",
                    width: "{display_width}",
                    height: "{display_height}",
                    loading: "lazy",
                    alt: "Terminal screenshot: {fixture_id}",
                    class: "screenshot-img",
                }
            }
        }
        None => rsx! {},
    }
}

/// Check if a screenshot exists for a specific provider for the given (theme, fixture).
pub fn has_screenshot_for_provider(
    manifest: &Option<ScreenshotManifest>,
    provider: &str,
    theme: &str,
    fixture: &str,
) -> bool {
    find_screenshot_url(manifest, provider, theme, fixture).is_some()
}

/// Find a screenshot URL for a specific provider + theme + fixture.
/// Returns `(full_url, width, height)`.
fn find_screenshot_url(
    manifest: &Option<ScreenshotManifest>,
    provider: &str,
    theme: &str,
    fixture: &str,
) -> Option<(String, u32, u32)> {
    let manifest = manifest.as_ref()?;
    let meta = manifest.find(provider, theme, fixture)?;
    let url = meta.cache_busted_url(&manifest.base_url);
    Some((url, meta.width, meta.height))
}

/// Find a screenshot URL from any provider for the given (theme, fixture).
/// Picks the first provider that has coverage. Returns `(full_url, width, height)`.
fn find_any_screenshot_url(
    manifest: &Option<ScreenshotManifest>,
    theme: &str,
    fixture: &str,
) -> Option<(String, u32, u32)> {
    let manifest = manifest.as_ref()?;
    // Try each provider in order; return the first match
    for provider in &manifest.providers {
        if let Some(meta) = manifest.find(&provider.slug, theme, fixture) {
            let url = meta.cache_busted_url(&manifest.base_url);
            return Some((url, meta.width, meta.height));
        }
    }
    None
}
