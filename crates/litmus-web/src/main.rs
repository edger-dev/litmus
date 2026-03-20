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

/// Theme listing page.
#[component]
fn ThemeList() -> Element {
    let all_themes = themes::load_embedded_themes();

    rsx! {
        div {
            h2 {
                style: "font-size: 1.3rem; margin-bottom: 1.5rem;",
                "Themes"
            }

            div { class: "theme-grid",
                for theme in &all_themes {
                    ThemeCard { theme: theme.clone() }
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

                        div { class: "swatch-row",
                            for (i, color) in theme.ansi.as_array().iter().enumerate() {
                                div {
                                    class: "swatch-lg mono",
                                    style: "background: {color.to_hex()}; color: {fg};",
                                    title: "{color.to_hex()}",
                                    "{i}"
                                }
                            }
                        }
                    }

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
