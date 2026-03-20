mod family;
mod scene_renderer;
mod themes;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Shell)]
    #[route("/")]
    ThemeList {},
    #[route("/theme/:slug")]
    ThemeDetail { slug: String },
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/style.css") }
        Router::<Route> {}
    }
}

/// Shared app shell: nav header + content area.
#[component]
fn Shell() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; background: #1a1b26; color: #c0caf5; \
                    font-family: system-ui, -apple-system, sans-serif;",

            nav { class: "nav",
                Link {
                    to: Route::ThemeList {},
                    style: "font-size: 1.25rem; font-weight: bold; letter-spacing: 0.02em;",
                    "litmus"
                }
                span {
                    style: "font-size: 0.85rem; opacity: 0.6;",
                    "terminal color theme previewer"
                }
            }

            div { class: "content",
                Outlet::<Route> {}
            }
        }
    }
}

/// Theme listing page grouped by family.
#[component]
fn ThemeList() -> Element {
    let all_themes = themes::load_embedded_themes();
    let families = family::group_by_family(&all_themes);

    rsx! {
        div {
            h2 {
                style: "font-size: 1.3rem; margin-bottom: 1.5rem;",
                "Themes"
            }

            for fam in &families {
                div {
                    style: "margin-bottom: 2rem;",

                    h3 {
                        style: "font-size: 1rem; margin-bottom: 0.75rem; opacity: 0.8;",
                        "{fam.name}"
                    }

                    div { class: "theme-grid",
                        for theme in &fam.themes {
                            ThemeCard { theme: theme.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// A clickable theme card showing name + color swatches.
#[component]
fn ThemeCard(theme: litmus_model::Theme) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let slug = theme.name.to_lowercase().replace(' ', "-");
    let ansi = theme.ansi.as_array();

    rsx! {
        Link {
            to: Route::ThemeDetail { slug: slug },
            style: "text-decoration: none; color: inherit;",

            div {
                class: "theme-card",
                style: "background: {bg}; color: {fg};",

                div {
                    style: "font-weight: bold; margin-bottom: 0.75rem; font-size: 0.95rem;",
                    "{theme.name}"
                }

                div { class: "swatch-row",
                    for color in ansi.iter() {
                        div {
                            class: "swatch",
                            style: "background: {color.to_hex()};",
                        }
                    }
                }
            }
        }
    }
}

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page.
#[component]
fn ThemeDetail(slug: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let theme = all_themes.iter().find(|t| {
        t.name.to_lowercase().replace(' ', "-") == slug
    });

    match theme {
        Some(theme) => {
            let theme = theme.clone();
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();

            // Contrast validation
            let issues = litmus_model::contrast::validate_theme_readability(&theme);
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );

            rsx! {
                div {
                    div {
                        style: "margin-bottom: 1.5rem;",
                        Link {
                            to: Route::ThemeList {},
                            style: "color: #7aa2f7; text-decoration: none; font-size: 0.9rem;",
                            "< All themes"
                        }
                    }

                    h2 {
                        style: "font-size: 1.3rem; margin-bottom: 0.5rem;",
                        "{theme.name}"
                    }

                    // Contrast summary
                    div {
                        style: "margin-bottom: 1.5rem; font-size: 0.85rem;",

                        span {
                            style: "opacity: 0.7; margin-right: 0.5rem;",
                            "fg/bg contrast: "
                        }
                        span {
                            class: "mono",
                            style: if fg_bg_ratio >= litmus_model::contrast::WCAG_AA_NORMAL {
                                "color: #a6e3a1;"
                            } else {
                                "color: #f38ba8;"
                            },
                            "{fg_bg_ratio:.1}:1"
                        }

                        if issues.is_empty() {
                            span {
                                style: "margin-left: 1.5rem; color: #a6e3a1;",
                                "All scene colors pass WCAG AA"
                            }
                        } else {
                            span {
                                style: "margin-left: 1.5rem; color: #f38ba8;",
                                "{issues.len()} contrast issue(s) in scene previews"
                            }
                        }
                    }

                    // Color palette
                    div {
                        class: "color-palette",
                        style: "background: {bg}; color: {fg};",

                        div {
                            style: "font-size: 0.85rem; font-weight: bold; margin-bottom: 0.5rem; \
                                    opacity: 0.7;",
                            "Color Palette"
                        }

                        div { class: "special-colors",
                            ColorSwatch { label: "bg", color: theme.background.to_hex() }
                            ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                            ColorSwatch { label: "cursor", color: theme.cursor.to_hex() }
                            ColorSwatch { label: "sel bg", color: theme.selection_background.to_hex() }
                            ColorSwatch { label: "sel fg", color: theme.selection_foreground.to_hex() }
                        }

                        // ANSI colors with names
                        div {
                            style: "display: grid; grid-template-columns: repeat(8, 1fr); gap: 0.5rem; \
                                    margin-top: 0.5rem;",
                            for (i, color) in theme.ansi.as_array().iter().enumerate() {
                                div {
                                    style: "text-align: center;",
                                    div {
                                        class: "swatch-lg mono",
                                        style: "background: {color.to_hex()}; color: {fg}; \
                                                width: 100%; margin-bottom: 0.25rem;",
                                        title: "{color.to_hex()}",
                                        "{i}"
                                    }
                                    div {
                                        class: "mono",
                                        style: "font-size: 0.55rem; opacity: 0.7; \
                                                white-space: nowrap; overflow: hidden; \
                                                text-overflow: ellipsis;",
                                        "{ANSI_NAMES[i]}"
                                    }
                                }
                            }
                        }
                    }

                    // Scene previews
                    scene_renderer::AllScenesView { theme: theme }
                }
            }
        }
        None => {
            rsx! {
                div {
                    h2 { "Theme not found" }
                    p { "No theme matches \"{slug}\"." }
                    Link {
                        to: Route::ThemeList {},
                        style: "color: #7aa2f7;",
                        "Back to all themes"
                    }
                }
            }
        }
    }
}

#[component]
fn ColorSwatch(label: String, color: String) -> Element {
    rsx! {
        div { class: "color-label",
            div {
                class: "color-chip",
                style: "background: {color};",
            }
            span { "{label}" }
        }
    }
}
