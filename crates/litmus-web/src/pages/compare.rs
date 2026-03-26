use dioxus::prelude::*;

use crate::components::ColorSwatch;
use crate::fixtures;
use crate::screenshot_view::{has_screenshot_for_provider, ScreenshotImage};
use crate::state::*;
use crate::term_renderer;
use crate::themes;
use crate::Route;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Multi-theme comparison (2-4 themes side by side).
#[component]
pub fn CompareThemes(provider: String, slugs: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let provider = active_provider.read().0.clone();
    let all_themes = themes::themes_for_provider(&provider);
    let all_fixtures = fixtures::all_fixtures();
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;
    let manifest_state = use_context::<Signal<ManifestState>>();
    let mut show_screenshots = use_signal(|| false);

    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).collect();

    let compare_themes: Vec<litmus_model::Theme> = slug_list
        .iter()
        .filter_map(|slug| all_themes.iter().find(|t| theme_slug(&t.name) == *slug).cloned())
        .map(|t| maybe_simulate(&t, cvd))
        .collect();

    if compare_themes.is_empty() {
        return rsx! {
            div {
                h2 { "No themes found" }
                p { "Could not find any matching themes." }
                Link { to: Route::ThemeList { provider: provider.clone() }, class: "accent-link", "Back to all themes" }
            }
        };
    }

    let n = compare_themes.len();
    let grid_cols = format!("repeat({n}, 1fr)");
    let screenshots_on = *show_screenshots.read();

    rsx! {
        div { class: "page-compare",
            style: "--compare-cols: {grid_cols};",

            // Toolbar: toggle + column headers
            div { class: "compare-toolbar",
                div { class: "compare-view-toggle",
                    button {
                        class: if !screenshots_on { "compare-toggle-btn compare-toggle-btn-active" } else { "compare-toggle-btn" },
                        onclick: move |_| show_screenshots.set(false),
                        "Simulated"
                    }
                    button {
                        class: if screenshots_on { "compare-toggle-btn compare-toggle-btn-active" } else { "compare-toggle-btn" },
                        onclick: move |_| show_screenshots.set(true),
                        "Screenshot"
                    }
                }
            }

            // Sticky column headers with theme names
            div { class: "compare-column-headers",
                for theme in &compare_themes {
                    div { class: "compare-column-header",
                        Link {
                            to: Route::ThemeDetail {
                                provider: provider.clone(),
                                slug: theme_slug(&theme.name),
                            },
                            class: "compare-column-header-link",
                            "{theme.name}"
                        }
                    }
                }
            }

            for fixture in all_fixtures {
                div { class: "compare-scene-group",
                    id: "scene-{fixture.id}",
                    h3 { class: "compare-scene-name", "{fixture.name}" }

                    div { class: "compare-grid",
                        for theme in &compare_themes {
                            div { class: "compare-grid-item",
                                if screenshots_on {
                                    {
                                        let t_slug = theme_slug(&theme.name);
                                        let has_screenshot = has_screenshot_for_provider(
                                            &manifest_state.read().0,
                                            &provider,
                                            &t_slug,
                                            &fixture.id,
                                        );
                                        if has_screenshot {
                                            rsx! {
                                                ScreenshotImage {
                                                    theme_slug: t_slug,
                                                    fixture_id: fixture.id.clone(),
                                                    provider: provider.clone(),
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                div { class: "compare-screenshot-placeholder",
                                                    "No screenshot"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    term_renderer::TermOutputView {
                                        theme: theme.clone(),
                                        output: fixture.clone(),
                                        compact: n > 2,
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Color palettes at the end
            h3 { class: "compare-scene-name", "Color Palette" }
            div { class: "compare-grid",
                for theme in &compare_themes {
                    {
                        let bg = theme.background.to_hex();
                        let fg = theme.foreground.to_hex();
                        rsx! {
                            div { class: "compare-grid-item",
                                div {
                                    class: "color-palette",
                                    style: "background: {bg}; color: {fg};",
                                    div { class: "palette-expanded",
                                        div { class: "special-colors",
                                            ColorSwatch { label: "bg".to_string(), color: theme.background.to_hex() }
                                            ColorSwatch { label: "fg".to_string(), color: theme.foreground.to_hex() }
                                            ColorSwatch { label: "cursor".to_string(), color: theme.cursor.to_hex() }
                                            ColorSwatch { label: "sel bg".to_string(), color: theme.selection_background.to_hex() }
                                            ColorSwatch { label: "sel fg".to_string(), color: theme.selection_foreground.to_hex() }
                                        }
                                        div { class: "ansi-grid",
                                            for (i, color) in theme.ansi.as_array().iter().enumerate() {
                                                div { class: "ansi-cell",
                                                    div {
                                                        class: "swatch-lg mono",
                                                        style: "background: {color.to_hex()}; color: {fg};",
                                                        title: "{color.to_hex()}",
                                                        "{i}"
                                                    }
                                                    div { class: "mono ansi-name", "{ANSI_NAMES[i]}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
