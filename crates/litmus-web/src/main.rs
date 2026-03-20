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
    rsx! { Router::<Route> {} }
}

/// Shared app shell: nav header + content area.
#[component]
fn Shell() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; background: #1a1b26; color: #c0caf5; \
                    font-family: system-ui, -apple-system, sans-serif;",

            // Nav header
            nav {
                style: "border-bottom: 1px solid #33467c; padding: 0.75rem 2rem; \
                        display: flex; align-items: center; gap: 1.5rem;",

                Link {
                    to: Route::ThemeList {},
                    style: "text-decoration: none; color: #c0caf5; font-size: 1.25rem; \
                            font-weight: bold; letter-spacing: 0.02em;",
                    "litmus"
                }

                span {
                    style: "font-size: 0.85rem; opacity: 0.6;",
                    "terminal color theme previewer"
                }
            }

            // Page content
            div {
                style: "max-width: 1100px; margin: 0 auto; padding: 2rem;",
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

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); \
                        gap: 1rem;",

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
                style: "background: {bg}; color: {fg}; border-radius: 0.5rem; \
                        padding: 1rem; border: 1px solid #33467c; \
                        transition: transform 0.15s ease; cursor: pointer;",
                onmouseenter: |_| {},

                // Theme name
                div {
                    style: "font-weight: bold; margin-bottom: 0.75rem; font-size: 0.95rem;",
                    "{theme.name}"
                }

                // Color swatches row
                div {
                    style: "display: flex; gap: 3px;",
                    for color in ansi.iter() {
                        div {
                            style: "width: 20px; height: 20px; border-radius: 3px; \
                                    background: {color.to_hex()};",
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
                    // Back link
                    div {
                        style: "margin-bottom: 1.5rem;",
                        Link {
                            to: Route::ThemeList {},
                            style: "color: #7aa2f7; text-decoration: none; font-size: 0.9rem;",
                            "< All themes"
                        }
                    }

                    // Theme header
                    h2 {
                        style: "font-size: 1.3rem; margin-bottom: 0.5rem;",
                        "{theme.name}"
                    }

                    // Color palette
                    div {
                        style: "background: {bg}; color: {fg}; border-radius: 0.5rem; \
                                padding: 1rem; margin-bottom: 1.5rem; border: 1px solid #33467c;",

                        div {
                            style: "font-size: 0.85rem; font-weight: bold; margin-bottom: 0.5rem; \
                                    opacity: 0.7;",
                            "Color Palette"
                        }

                        // Special colors
                        div {
                            style: "display: flex; gap: 0.5rem; margin-bottom: 0.75rem; \
                                    flex-wrap: wrap;",
                            ColorSwatch { label: "bg", color: theme.background.to_hex() }
                            ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                            ColorSwatch { label: "cursor", color: theme.cursor.to_hex() }
                            ColorSwatch { label: "sel bg", color: theme.selection_background.to_hex() }
                            ColorSwatch { label: "sel fg", color: theme.selection_foreground.to_hex() }
                        }

                        // ANSI colors
                        div {
                            style: "display: flex; gap: 3px; flex-wrap: wrap;",
                            for (i, color) in theme.ansi.as_array().iter().enumerate() {
                                div {
                                    style: "width: 32px; height: 32px; border-radius: 4px; \
                                            background: {color.to_hex()}; display: flex; \
                                            align-items: center; justify-content: center; \
                                            font-size: 0.6rem; color: {fg};",
                                    title: "{color.to_hex()}",
                                    "{i}"
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
        div {
            style: "display: flex; align-items: center; gap: 0.3rem; \
                    font-size: 0.75rem; font-family: monospace;",
            div {
                style: "width: 16px; height: 16px; border-radius: 3px; \
                        background: {color}; border: 1px solid rgba(255,255,255,0.2);",
            }
            span { "{label}" }
        }
    }
}
