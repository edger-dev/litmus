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

/// Filter mode for light/dark themes.
#[derive(Clone, Copy, PartialEq)]
enum VariantFilter {
    All,
    Dark,
    Light,
}

fn is_light_theme(theme: &litmus_model::Theme) -> bool {
    litmus_model::contrast::relative_luminance(&theme.background) > 0.5
}

fn theme_passes_filter(
    theme: &litmus_model::Theme,
    variant: VariantFilter,
    good_contrast_only: bool,
    query: &str,
) -> bool {
    if !query.is_empty() {
        let q = query.to_lowercase();
        let name = theme.name.to_lowercase();
        let fam = family::theme_family(&theme.name).to_lowercase();
        if !name.contains(&q) && !fam.contains(&q) {
            return false;
        }
    }
    match variant {
        VariantFilter::All => {}
        VariantFilter::Dark => {
            if is_light_theme(theme) {
                return false;
            }
        }
        VariantFilter::Light => {
            if !is_light_theme(theme) {
                return false;
            }
        }
    }
    if good_contrast_only {
        let issues = litmus_model::contrast::validate_theme_readability(theme);
        if !issues.is_empty() {
            return false;
        }
    }
    true
}

/// Home page with dual navigation: browse by theme or by scene.
#[component]
fn ThemeList() -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();

    let mut variant_filter = use_signal(|| VariantFilter::All);
    let mut good_contrast = use_signal(|| false);
    let mut search_query = use_signal(String::new);

    let variant = *variant_filter.read();
    let contrast_on = *good_contrast.read();
    let query = search_query.read().clone();

    let filtered: Vec<litmus_model::Theme> = all_themes
        .iter()
        .filter(|t| theme_passes_filter(t, variant, contrast_on, &query))
        .cloned()
        .collect();
    let families = family::group_by_family(&filtered);
    let total = all_themes.len();
    let shown = filtered.len();

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
            div {
                style: "display: flex; justify-content: space-between; align-items: center; \
                        margin-bottom: 1.5rem; flex-wrap: wrap; gap: 0.75rem;",

                h2 {
                    style: "font-size: 1.3rem;",
                    "Browse by Theme"
                }

                // Filter controls
                div { class: "filter-bar",
                    // Search
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search themes...",
                        value: "{query}",
                        oninput: move |evt: Event<FormData>| {
                            search_query.set(evt.value());
                        },
                    }

                    // Variant filter
                    FilterButton {
                        label: "All",
                        active: variant == VariantFilter::All,
                        onclick: move |_| variant_filter.set(VariantFilter::All),
                    }
                    FilterButton {
                        label: "Dark",
                        active: variant == VariantFilter::Dark,
                        onclick: move |_| variant_filter.set(VariantFilter::Dark),
                    }
                    FilterButton {
                        label: "Light",
                        active: variant == VariantFilter::Light,
                        onclick: move |_| variant_filter.set(VariantFilter::Light),
                    }

                    // Divider
                    span { style: "opacity: 0.3; margin: 0 0.25rem;", "|" }

                    // Contrast filter
                    FilterButton {
                        label: "Good contrast",
                        active: contrast_on,
                        onclick: move |_| good_contrast.set(!contrast_on),
                    }

                    // Count
                    if shown < total {
                        span {
                            style: "font-size: 0.8rem; opacity: 0.5; margin-left: 0.5rem;",
                            "{shown}/{total}"
                        }
                    }
                }
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

            if families.is_empty() {
                p {
                    style: "opacity: 0.5; text-align: center; padding: 2rem;",
                    "No themes match the current filters."
                }
            }
        }
    }
}

#[component]
fn FilterButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let style = if active {
        "background: rgba(122, 162, 247, 0.2); color: #7aa2f7; border-color: #7aa2f7;"
    } else {
        "background: transparent; color: inherit; border-color: rgba(255,255,255,0.15);"
    };

    rsx! {
        button {
            class: "filter-btn",
            style: "{style}",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// A clickable theme card showing name, mini scene preview, and color swatches.
#[component]
fn ThemeCard(theme: litmus_model::Theme) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let slug = theme_slug(&theme.name);
    let ansi = theme.ansi.as_array();
    let is_light = litmus_model::contrast::relative_luminance(&theme.background) > 0.5;
    let variant = if is_light { "light" } else { "dark" };
    let fg_bg_ratio = litmus_model::contrast::contrast_ratio(&theme.foreground, &theme.background);
    let preview_scene = litmus_model::scenes::shell_prompt_scene();

    rsx! {
        Link {
            to: Route::ThemeDetail { slug },
            style: "text-decoration: none; color: inherit;",

            div {
                class: "theme-card",
                style: "background: {bg}; color: {fg};",

                // Header: name + metadata
                div {
                    style: "display: flex; justify-content: space-between; align-items: baseline;",
                    span {
                        style: "font-weight: bold; font-size: 0.95rem;",
                        "{theme.name}"
                    }
                    span {
                        style: "font-size: 0.7rem; opacity: 0.6;",
                        "{variant} {fg_bg_ratio:.1}:1"
                    }
                }

                // Mini scene preview
                scene_renderer::ScenePreview {
                    theme: theme.clone(),
                    scene: preview_scene,
                    max_lines: 5,
                }

                // Color swatches
                div { class: "swatch-row",
                    style: "margin-top: 0.5rem;",
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

/// Single theme detail page with tabbed scene navigation.
#[component]
fn ThemeDetail(slug: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let theme = all_themes.iter().find(|t| theme_slug(&t.name) == slug);
    let mut active_tab = use_signal(|| 0usize);
    let mut palette_expanded = use_signal(|| false);

    match theme {
        Some(theme) => {
            let theme = theme.clone();
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();
            let this_slug = theme_slug(&theme.name);
            let scenes = litmus_model::scenes::all_scenes();
            let tab_idx = (*active_tab.read()).min(scenes.len().saturating_sub(1));
            let expanded = *palette_expanded.read();

            // Pick a default comparison partner
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
                    // Breadcrumb + actions
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

                    // Theme header with inline metadata
                    div {
                        style: "display: flex; align-items: baseline; gap: 1rem; \
                                margin-bottom: 1rem; flex-wrap: wrap;",
                        h2 {
                            style: "font-size: 1.3rem;",
                            "{theme.name}"
                        }
                        span {
                            class: "mono",
                            style: "font-size: 0.8rem; opacity: 0.7;",
                            if fg_bg_ratio >= litmus_model::contrast::WCAG_AA_NORMAL {
                                span { style: "color: #a6e3a1;", "{fg_bg_ratio:.1}:1" }
                            } else {
                                span { style: "color: #f38ba8;", "{fg_bg_ratio:.1}:1" }
                            }
                        }
                        if !issues.is_empty() {
                            span {
                                style: "font-size: 0.8rem; color: #f38ba8;",
                                "{issues.len()} contrast issue(s)"
                            }
                        }
                    }

                    // Compact color palette (expandable)
                    div {
                        class: "color-palette",
                        style: "background: {bg}; color: {fg}; margin-bottom: 1.5rem;",

                        // Compact: single row of all colors
                        div {
                            style: "display: flex; align-items: center; gap: 0.5rem; \
                                    cursor: pointer;",
                            onclick: move |_| palette_expanded.set(!expanded),

                            // Special colors
                            ColorSwatch { label: "bg", color: theme.background.to_hex() }
                            ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                            ColorSwatch { label: "cur", color: theme.cursor.to_hex() }

                            // Divider
                            span { style: "opacity: 0.2;", "|" }

                            // ANSI strip
                            div { class: "swatch-row",
                                for color in theme.ansi.as_array().iter() {
                                    div {
                                        class: "swatch",
                                        style: "background: {color.to_hex()};",
                                        title: "{color.to_hex()}",
                                    }
                                }
                            }

                            span {
                                class: "mono",
                                style: "font-size: 0.7rem; opacity: 0.5; margin-left: auto;",
                                if expanded { "collapse" } else { "expand" }
                            }
                        }

                        // Expanded: full detail
                        if expanded {
                            div {
                                style: "margin-top: 1rem;",

                                div { class: "special-colors",
                                    style: "margin-bottom: 0.75rem;",
                                    ColorSwatch { label: "bg", color: theme.background.to_hex() }
                                    ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                                    ColorSwatch { label: "cursor", color: theme.cursor.to_hex() }
                                    ColorSwatch { label: "sel bg", color: theme.selection_background.to_hex() }
                                    ColorSwatch { label: "sel fg", color: theme.selection_foreground.to_hex() }
                                }

                                div {
                                    style: "display: grid; grid-template-columns: repeat(8, 1fr); \
                                            gap: 0.5rem;",
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
                                                style: "font-size: 0.55rem; opacity: 0.7;",
                                                "{ANSI_NAMES[i]}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Scene tabs
                    div { class: "scene-tabs",
                        for (i, scene) in scenes.iter().enumerate() {
                            button {
                                class: if i == tab_idx { "scene-tab scene-tab-active" } else { "scene-tab" },
                                onclick: move |_| active_tab.set(i),
                                "{scene.name}"
                            }
                        }
                    }

                    // Active scene
                    if let Some(scene) = scenes.get(tab_idx) {
                        scene_renderer::SceneView {
                            theme: theme.clone(),
                            scene: scene.clone(),
                        }
                    }
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
