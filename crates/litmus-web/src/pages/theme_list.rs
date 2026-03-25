use dioxus::prelude::*;

use crate::components::{ScoreRing, ShortlistCheckbox, UseAsAppThemeButton};
use crate::fixtures;
use crate::scene_renderer;
use crate::term_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Home page: theme card grid with inline filters.
#[component]
pub fn ThemeList() -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let all_themes = themes::themes_for_provider(&active_provider.read().0);
    let mut filter = use_signal(FilterState::default);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;

    let filter_val = filter.read().clone();

    let mut filtered: Vec<litmus_model::Theme> = all_themes
        .iter()
        .filter(|t| theme_passes_filter(t, &filter_val))
        .map(|t| maybe_simulate(t, cvd))
        .collect();
    filtered.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let total = all_themes.len();
    let shown = filtered.len();

    // Count badges for variant filter
    let count_all = all_themes.len();
    let count_dark = all_themes.iter().filter(|t| !is_light_theme(t)).count();
    let count_light = all_themes.iter().filter(|t| is_light_theme(t)).count();

    let query = filter_val.query.clone();
    let variant = filter_val.variant;
    let min_read = filter_val.min_readability;

    rsx! {
        div { class: "page-theme-list",
            div { class: "page-header",
                h2 { class: "page-title", "Browse Themes" }
                if shown < total {
                    span { class: "filter-count", "{shown}/{total}" }
                }
            }

            // Inline filter bar
            div { class: "filter-bar",
                input {
                    class: "filter-bar-search",
                    r#type: "text",
                    placeholder: "Search themes...",
                    value: "{query}",
                    oninput: move |evt: Event<FormData>| {
                        filter.write().query = evt.value();
                    },
                }

                button {
                    class: if variant == VariantFilter::All { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::All { "true" } else { "false" },
                    onclick: move |_| filter.write().variant = VariantFilter::All,
                    "All "
                    span { class: "filter-badge", "({count_all})" }
                }
                button {
                    class: if variant == VariantFilter::Dark { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::Dark { "true" } else { "false" },
                    onclick: move |_| filter.write().variant = VariantFilter::Dark,
                    "Dark "
                    span { class: "filter-badge", "({count_dark})" }
                }
                button {
                    class: if variant == VariantFilter::Light { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::Light { "true" } else { "false" },
                    onclick: move |_| filter.write().variant = VariantFilter::Light,
                    "Light "
                    span { class: "filter-badge", "({count_light})" }
                }

                div { class: "filter-bar-readability-wrap",
                    select {
                        class: "filter-bar-readability",
                        value: match min_read {
                            None => "any",
                            Some(v) => match v {
                                80 => "80",
                                90 => "90",
                                95 => "95",
                                _ => "any",
                            },
                        },
                        onchange: move |evt: Event<FormData>| {
                            let val = evt.value();
                            filter.write().min_readability = match val.as_str() {
                                "80" => Some(80),
                                "90" => Some(90),
                                "95" => Some(95),
                                _ => None,
                            };
                        },
                        option { value: "any", "Readability: Any" }
                        option { value: "80", "Readability: 80%+" }
                        option { value: "90", "Readability: 90%+" }
                        option { value: "95", "Readability: 95%+" }
                    }
                }
            }

            // Flat grid — no family groups
            div { class: "theme-grid",
                for theme in &filtered {
                    ThemeCard { theme: theme.clone() }
                }
            }

            if filtered.is_empty() {
                p { class: "empty-state", "No themes match the current filters." }
            }
        }
    }
}

#[component]
fn ThemeCard(theme: litmus_model::Theme) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let slug = theme_slug(&theme.name);
    let readability = litmus_model::contrast::readability_score(&theme) as u8;
    let preview_fixture = fixtures::default_fixture();

    rsx! {
        div {
            class: "theme-card",
            style: "background: {bg}; color: {fg};",

            Link {
                to: Route::ThemeDetail { slug: slug.clone() },
                class: "theme-card-link",

                div { class: "theme-card-body",
                    div { class: "theme-card-header",
                        span { class: "theme-card-name", "{theme.name}" }
                    }

                    div { class: "theme-card-preview",
                        if let Some(fixture) = preview_fixture {
                            term_renderer::TermOutputPreview {
                                theme: theme.clone(),
                                output: fixture.clone(),
                                max_lines: 5,
                            }
                        } else {
                            scene_renderer::ScenePreview {
                                theme: theme.clone(),
                                scene: litmus_model::scenes::shell_prompt_scene(),
                                max_lines: 5,
                            }
                        }

                        div { class: "swatch-row",
                            for color in theme.ansi.as_array().iter() {
                                div {
                                    class: "swatch",
                                    style: "background: {color.to_hex()};",
                                }
                            }
                        }
                    }
                }
            }

            div { class: "theme-card-actions",
                span { class: "theme-card-score", title: "Readability score",
                    ScoreRing { score: readability, size: 28.0 }
                }
                span { class: "theme-card-actions-right",
                    ShortlistCheckbox { slug: slug.clone(), name: theme.name.clone() }
                    UseAsAppThemeButton { slug }
                }
            }
        }
    }
}
