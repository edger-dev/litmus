use dioxus::prelude::*;
use litmus_model::scene::Scene;
use litmus_model::screenshot::ScreenshotManifest;
use litmus_model::Theme;

use crate::scene_renderer::{ScenePreview, SceneView};
use crate::state::ManifestState;

/// Display a real terminal screenshot from the CDN.
///
/// Falls back to the simulated `SceneView` if:
/// - The manifest is not loaded yet
/// - No screenshot exists for this (provider, theme, fixture) combination
/// - The image fails to load (`onerror`)
#[component]
pub fn ScreenshotSceneView(
    /// Provider slug (e.g. "kitty")
    provider: String,
    /// Theme slug (e.g. "tokyo-night")
    theme_slug: String,
    /// Fixture id (e.g. "git-diff")
    fixture_id: String,
    /// Fallback simulated theme (used when no screenshot is available)
    fallback_theme: Theme,
    /// Fallback simulated scene (used when no screenshot is available)
    fallback_scene: Scene,
    /// If true, render as compact preview (first 5 lines for simulated fallback)
    #[props(default = false)]
    compact: bool,
) -> Element {
    let manifest_state = use_context::<Signal<ManifestState>>();
    let manifest = manifest_state.read();

    match find_screenshot_url(&manifest.0, &provider, &theme_slug, &fixture_id) {
        Some((url, width, height)) => {
            // Render the real screenshot image
            let display_width = width / 2; // 2x resolution → display at 1x
            let display_height = height / 2;
            let fallback_theme_clone = fallback_theme.clone();
            let fallback_scene_clone = fallback_scene.clone();

            rsx! {
                div { class: "scene-block screenshot-block",
                    img {
                        src: "{url}",
                        width: "{display_width}",
                        height: "{display_height}",
                        loading: "lazy",
                        alt: "Terminal screenshot: {fixture_id} with provider {provider}",
                        class: "screenshot-img",
                        // On load error, the img is hidden via CSS; the fallback is shown
                        onerror: move |_| {
                            // Can't easily swap to fallback in Dioxus without state;
                            // CSS hides failed images and shows the noscript-style fallback.
                        }
                    }
                    // Visually-hidden fallback shown by CSS when img fails to load
                    div { class: "screenshot-fallback",
                        if compact {
                            ScenePreview {
                                theme: fallback_theme_clone,
                                scene: fallback_scene_clone,
                            }
                        } else {
                            SceneView {
                                theme: fallback_theme_clone,
                                scene: fallback_scene_clone,
                            }
                        }
                    }
                }
            }
        }
        None => {
            // No screenshot available — use simulated rendering
            rsx! {
                if compact {
                    ScenePreview {
                        theme: fallback_theme,
                        scene: fallback_scene,
                    }
                } else {
                    SceneView {
                        theme: fallback_theme,
                        scene: fallback_scene,
                    }
                }
            }
        }
    }
}

/// A row of provider selector pills.
///
/// Shows "Simulated" plus any providers that have screenshots for the given theme.
#[component]
pub fn ProviderSelector(
    /// Current theme slug — used to filter which providers have coverage
    theme_slug: String,
    /// Currently active provider slug
    active: String,
    /// Callback when the user selects a different provider
    on_change: EventHandler<String>,
) -> Element {
    let manifest_state = use_context::<Signal<ManifestState>>();
    let manifest = manifest_state.read();

    // Build the list of available providers: always include "Simulated",
    // then add any providers that have at least one screenshot for this theme.
    let mut providers: Vec<(String, String)> = vec![
        ("simulated".to_string(), "Simulated".to_string()),
    ];

    if let Some(manifest) = &manifest.0 {
        let theme_screenshots = manifest.for_theme(&theme_slug);
        let mut seen = std::collections::HashSet::new();
        for screenshot in theme_screenshots {
            let slug = &screenshot.key.provider;
            if seen.insert(slug.clone()) {
                let name = manifest
                    .provider(slug)
                    .map(|p| p.name.as_str())
                    .unwrap_or(slug.as_str())
                    .to_string();
                providers.push((slug.clone(), name));
            }
        }
    }

    // Don't render the selector if only "Simulated" is available
    if providers.len() <= 1 {
        return rsx! {};
    }

    rsx! {
        div { class: "provider-selector",
            span { class: "provider-selector-label", "Rendering:" }
            div { class: "provider-pills",
                for (slug, name) in providers {
                    {
                        let is_active = slug == active;
                        let click_slug = slug.clone();
                        rsx! {
                            button {
                                class: if is_active { "provider-pill provider-pill-active" } else { "provider-pill" },
                                onclick: move |_| on_change.call(click_slug.clone()),
                                "{name}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Find a screenshot URL for the given key, with cache-busting.
/// Returns `(full_url, width, height)` or None if not found.
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

/// Map a scene ID from the simulated system to a fixture ID in the screenshot system.
/// Returns None if there is no corresponding fixture.
pub fn scene_id_to_fixture_id(scene_id: &str) -> Option<&'static str> {
    match scene_id {
        "shell-prompt" => Some("shell-prompt"),
        "git-diff" => Some("git-diff"),
        "ls-color" => Some("ls-color"),
        "cargo-build" => Some("cargo-build"),
        "log-viewer" => Some("git-log"),
        "python-repl" => Some("python-repl"),
        "htop" => Some("htop"),
        "neovim" => None, // Deferred to silo support (M-Next-5)
        _ => None,
    }
}
