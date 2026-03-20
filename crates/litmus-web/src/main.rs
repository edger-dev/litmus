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
    #[route("/scene/:scene_id")]
    SceneAcrossThemes { scene_id: String },
    #[route("/compare/:left/:right")]
    CompareThemes { left: String, right: String },
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

/// Home page with dual navigation: browse by theme or by scene.
#[component]
fn ThemeList() -> Element {
    let all_themes = themes::load_embedded_themes();
    let families = family::group_by_family(&all_themes);
    let scenes = litmus_model::scenes::all_scenes();

    rsx! {
        div {
            // Browse by Scene section
            div {
                style: "margin-bottom: 2.5rem;",

                h2 {
                    style: "font-size: 1.3rem; margin-bottom: 0.75rem;",
                    "Browse by Scene"
                }
                p {
                    style: "font-size: 0.85rem; opacity: 0.7; margin-bottom: 1rem;",
                    "See how all themes render a specific terminal context."
                }

                div {
                    style: "display: flex; gap: 0.75rem; flex-wrap: wrap;",
                    for scene in &scenes {
                        Link {
                            to: Route::SceneAcrossThemes { scene_id: scene.id.clone() },
                            class: "theme-card",
                            style: "text-decoration: none; color: inherit; padding: 0.75rem 1rem; \
                                    background: rgba(255,255,255,0.05); display: inline-block;",
                            div {
                                style: "font-weight: bold; font-size: 0.9rem; margin-bottom: 0.2rem;",
                                "{scene.name}"
                            }
                            div {
                                style: "font-size: 0.75rem; opacity: 0.6;",
                                "{scene.description}"
                            }
                        }
                    }
                }
            }

            // Browse by Theme section
            h2 {
                style: "font-size: 1.3rem; margin-bottom: 1.5rem;",
                "Browse by Theme"
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
    let slug = theme_slug(&theme.name);
    let ansi = theme.ansi.as_array();
    let is_light = litmus_model::contrast::relative_luminance(&theme.background) > 0.5;
    let variant = if is_light { "light" } else { "dark" };
    let fg_bg_ratio = litmus_model::contrast::contrast_ratio(&theme.foreground, &theme.background);

    rsx! {
        Link {
            to: Route::ThemeDetail { slug: slug },
            style: "text-decoration: none; color: inherit;",

            div {
                class: "theme-card",
                style: "background: {bg}; color: {fg};",

                div {
                    style: "display: flex; justify-content: space-between; align-items: baseline; \
                            margin-bottom: 0.75rem;",
                    span {
                        style: "font-weight: bold; font-size: 0.95rem;",
                        "{theme.name}"
                    }
                    span {
                        style: "font-size: 0.7rem; opacity: 0.6;",
                        "{variant} {fg_bg_ratio:.1}:1"
                    }
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
    let theme = all_themes.iter().find(|t| theme_slug(&t.name) == slug);

    match theme {
        Some(theme) => {
            let theme = theme.clone();
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();
            let this_slug = theme_slug(&theme.name);

            // Pick a default comparison partner (next theme in list, wrapping)
            let compare_partner = all_themes.iter()
                .find(|t| theme_slug(&t.name) != this_slug)
                .map(|t| theme_slug(&t.name))
                .unwrap_or_default();

            // Contrast validation
            let issues = litmus_model::contrast::validate_theme_readability(&theme);
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );

            rsx! {
                div {
                    div {
                        style: "margin-bottom: 1.5rem; display: flex; gap: 1.5rem;",
                        Link {
                            to: Route::ThemeList {},
                            style: "color: #7aa2f7; text-decoration: none; font-size: 0.9rem;",
                            "< All themes"
                        }
                        Link {
                            to: Route::CompareThemes {
                                left: this_slug,
                                right: compare_partner,
                            },
                            style: "color: #7aa2f7; text-decoration: none; font-size: 0.9rem;",
                            "Compare..."
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

fn theme_slug(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}

/// Side-by-side theme comparison.
#[component]
fn CompareThemes(left: String, right: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();

    let left_theme = all_themes.iter().find(|t| theme_slug(&t.name) == left);
    let right_theme = all_themes.iter().find(|t| theme_slug(&t.name) == right);

    let (Some(left_theme), Some(right_theme)) = (left_theme, right_theme) else {
        return rsx! {
            div {
                h2 { "Theme not found" }
                p { "Could not find one or both themes." }
                Link {
                    to: Route::ThemeList {},
                    style: "color: #7aa2f7;",
                    "Back to all themes"
                }
            }
        };
    };

    let left_theme = left_theme.clone();
    let right_theme = right_theme.clone();

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
                style: "font-size: 1.3rem; margin-bottom: 1.5rem;",
                "{left_theme.name} vs {right_theme.name}"
            }

            // Theme selectors for changing comparison
            CompareSelector {
                all_themes: all_themes.clone(),
                current_left: left.clone(),
                current_right: right.clone(),
            }

            for scene in &scenes {
                div {
                    style: "margin-bottom: 2rem;",

                    h3 {
                        style: "font-size: 0.95rem; margin-bottom: 0.75rem; opacity: 0.8;",
                        "{scene.name}"
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;",
                        class: "compare-grid",

                        div {
                            div {
                                style: "font-size: 0.8rem; margin-bottom: 0.25rem; opacity: 0.7;",
                                "{left_theme.name}"
                            }
                            scene_renderer::SceneView {
                                theme: left_theme.clone(),
                                scene: scene.clone(),
                            }
                        }
                        div {
                            div {
                                style: "font-size: 0.8rem; margin-bottom: 0.25rem; opacity: 0.7;",
                                "{right_theme.name}"
                            }
                            scene_renderer::SceneView {
                                theme: right_theme.clone(),
                                scene: scene.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Dropdowns for selecting comparison themes.
#[component]
fn CompareSelector(
    all_themes: Vec<litmus_model::Theme>,
    current_left: String,
    current_right: String,
) -> Element {
    let nav = use_navigator();
    let themes_for_right = all_themes.clone();

    let mut left_slug = use_signal(|| current_left.clone());
    let mut right_slug = use_signal(|| current_right.clone());

    rsx! {
        div {
            style: "display: flex; gap: 1rem; margin-bottom: 1.5rem; flex-wrap: wrap; \
                    align-items: center;",

            select {
                style: "background: #1a1b26; color: #c0caf5; border: 1px solid #33467c; \
                        padding: 0.4rem 0.6rem; border-radius: 0.25rem;",
                value: "{current_left}",
                onchange: move |evt: Event<FormData>| {
                    left_slug.set(evt.value());
                    nav.push(Route::CompareThemes {
                        left: evt.value(),
                        right: right_slug.read().clone(),
                    });
                },
                for t in &all_themes {
                    option {
                        value: "{theme_slug(&t.name)}",
                        "{t.name}"
                    }
                }
            }

            span { style: "opacity: 0.6;", "vs" }

            select {
                style: "background: #1a1b26; color: #c0caf5; border: 1px solid #33467c; \
                        padding: 0.4rem 0.6rem; border-radius: 0.25rem;",
                value: "{current_right}",
                onchange: move |evt: Event<FormData>| {
                    right_slug.set(evt.value());
                    nav.push(Route::CompareThemes {
                        left: left_slug.read().clone(),
                        right: evt.value(),
                    });
                },
                for t in &themes_for_right {
                    option {
                        value: "{theme_slug(&t.name)}",
                        "{t.name}"
                    }
                }
            }
        }
    }
}

/// Scene-centric view: one scene rendered across all themes.
#[component]
fn SceneAcrossThemes(scene_id: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();
    let scene = scenes.iter().find(|s| s.id == scene_id);

    match scene {
        Some(scene) => {
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
                        style: "font-size: 1.3rem; margin-bottom: 0.25rem;",
                        "{scene.name}"
                    }
                    p {
                        style: "font-size: 0.85rem; opacity: 0.7; margin-bottom: 1.5rem;",
                        "{scene.description}"
                    }

                    div { class: "scenes-container",
                        for theme in &all_themes {
                            div {
                                div {
                                    style: "font-size: 0.85rem; font-weight: bold; \
                                            margin-bottom: 0.25rem;",
                                    Link {
                                        to: Route::ThemeDetail {
                                            slug: theme.name.to_lowercase().replace(' ', "-"),
                                        },
                                        style: "color: #7aa2f7; text-decoration: none;",
                                        "{theme.name}"
                                    }
                                }
                                scene_renderer::SceneView {
                                    theme: theme.clone(),
                                    scene: scene.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div {
                    h2 { "Scene not found" }
                    p { "No scene matches \"{scene_id}\"." }
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
