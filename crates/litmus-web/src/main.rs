mod family;
mod scene_renderer;
mod themes;

use dioxus::prelude::*;
use dioxus::document::eval;

/// Global compare selection state — stores slugs of themes selected for comparison.
#[derive(Clone, Default)]
struct CompareSelection(Vec<String>);

const MAX_COMPARE: usize = 4;

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
    #[route("/compare/:slugs")]
    CompareThemes { slugs: String },
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(CompareSelection::default()));

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

            CompareBar {}
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
            to: Route::ThemeDetail { slug: slug.clone() },
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

                // Color swatches + compare
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; \
                            margin-top: 0.5rem;",
                    div { class: "swatch-row",
                        for color in ansi.iter() {
                            div {
                                class: "swatch",
                                style: "background: {color.to_hex()};",
                            }
                        }
                    }
                    CompareToggle { slug, name: theme.name.clone() }
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

            let scene_count = scenes.len();
            let mut compare_sel = use_context::<Signal<CompareSelection>>();
            let detail_slug = this_slug.clone();

            rsx! {
                div {
                    tabindex: "0",
                    autofocus: true,
                    style: "outline: none;",
                    onkeydown: move |evt: Event<KeyboardData>| {
                        match evt.key() {
                            Key::ArrowLeft => {
                                if tab_idx > 0 {
                                    active_tab.set(tab_idx - 1);
                                }
                            }
                            Key::ArrowRight => {
                                if tab_idx + 1 < scene_count {
                                    active_tab.set(tab_idx + 1);
                                }
                            }
                            Key::Character(ref c) if c == "c" => {
                                let mut sel = compare_sel.write();
                                if let Some(pos) = sel.0.iter().position(|s| s == &detail_slug) {
                                    sel.0.remove(pos);
                                } else if sel.0.len() < MAX_COMPARE {
                                    sel.0.push(detail_slug.clone());
                                }
                            }
                            _ => {}
                        }
                    },
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
                                slugs: format!("{},{}", this_slug.clone(), compare_partner),
                            },
                            style: "color: #7aa2f7; text-decoration: none; font-size: 0.9rem;",
                            "Compare..."
                        }
                        CompareToggle { slug: this_slug, name: theme.name.clone() }
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
                    div {
                        style: "display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; \
                                margin-bottom: 1rem;",
                        div { class: "scene-tabs",
                            for (i, scene) in scenes.iter().enumerate() {
                                button {
                                    class: if i == tab_idx { "scene-tab scene-tab-active" } else { "scene-tab" },
                                    onclick: move |_| active_tab.set(i),
                                    "{scene.name}"
                                }
                            }
                        }
                        span {
                            class: "mono",
                            style: "font-size: 0.65rem; opacity: 0.35; margin-left: auto;",
                            "← → navigate · c compare"
                        }
                    }

                    // Active scene
                    if let Some(scene) = scenes.get(tab_idx) {
                        scene_renderer::SceneView {
                            theme: theme.clone(),
                            scene: scene.clone(),
                        }
                    }

                    // Export section
                    ExportButtons { theme: theme.clone() }
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

/// Button to add/remove a theme from the compare selection.
#[component]
fn CompareToggle(slug: String, name: String) -> Element {
    let mut selection = use_context::<Signal<CompareSelection>>();
    let is_selected = selection.read().0.contains(&slug);

    let slug_for_click = slug.clone();
    rsx! {
        button {
            class: if is_selected { "compare-toggle compare-toggle-active" } else { "compare-toggle" },
            onclick: move |evt: Event<MouseData>| {
                evt.stop_propagation();
                let mut sel = selection.write();
                if let Some(pos) = sel.0.iter().position(|s| s == &slug_for_click) {
                    sel.0.remove(pos);
                } else if sel.0.len() < MAX_COMPARE {
                    sel.0.push(slug_for_click.clone());
                }
            },
            if is_selected { "- Remove" } else { "+ Compare" }
        }
    }
}

/// Floating bar showing selected themes for comparison.
#[component]
fn CompareBar() -> Element {
    let mut selection = use_context::<Signal<CompareSelection>>();
    let sel = selection.read().clone();
    let all_themes = themes::load_embedded_themes();

    if sel.0.is_empty() {
        return rsx! {};
    }

    let count = sel.0.len();
    let can_compare = count >= 2;

    rsx! {
        div { class: "compare-bar",
            div { class: "compare-bar-inner",
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;",
                    span {
                        style: "font-size: 0.85rem; font-weight: bold;",
                        "Compare ({count})"
                    }
                    for slug in &sel.0 {
                        {
                            let name = all_themes.iter()
                                .find(|t| theme_slug(&t.name) == *slug)
                                .map(|t| t.name.clone())
                                .unwrap_or_else(|| slug.clone());
                            let slug_remove = slug.clone();
                            rsx! {
                                span { class: "compare-chip",
                                    "{name}"
                                    button {
                                        class: "compare-chip-remove",
                                        onclick: move |_| {
                                            let mut sel = selection.write();
                                            sel.0.retain(|s| s != &slug_remove);
                                        },
                                        "x"
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    style: "display: flex; gap: 0.5rem;",
                    if can_compare {
                        Link {
                            to: Route::CompareThemes {
                                slugs: sel.0.join(","),
                            },
                            class: "compare-bar-btn",
                            "Go to Compare"
                        }
                    }
                    button {
                        class: "compare-bar-btn-clear",
                        onclick: move |_| {
                            selection.write().0.clear();
                        },
                        "Clear"
                    }
                }
            }
        }
    }
}

/// Export format selector with copy-to-clipboard buttons.
#[component]
fn ExportButtons(theme: litmus_model::Theme) -> Element {
    let mut active_format = use_signal(|| Option::<&'static str>::None);
    let mut copied = use_signal(|| false);

    let format = *active_format.read();
    let is_copied = *copied.read();

    let content = format.map(|f| match f {
        "kitty" => litmus_model::export::to_kitty_conf(&theme),
        "toml" => litmus_model::export::to_toml(&theme),
        "nix" => litmus_model::export::to_nix(&theme),
        _ => String::new(),
    });

    rsx! {
        div {
            style: "margin-top: 2rem; border-top: 1px solid rgba(255,255,255,0.1); \
                    padding-top: 1.5rem;",

            div {
                style: "display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap; \
                        margin-bottom: 1rem;",

                h3 {
                    style: "font-size: 1rem; margin: 0;",
                    "Export"
                }

                ExportFormatBtn {
                    label: "kitty.conf",
                    active: format == Some("kitty"),
                    onclick: move |_| {
                        active_format.set(Some("kitty"));
                        copied.set(false);
                    },
                }
                ExportFormatBtn {
                    label: "TOML",
                    active: format == Some("toml"),
                    onclick: move |_| {
                        active_format.set(Some("toml"));
                        copied.set(false);
                    },
                }
                ExportFormatBtn {
                    label: "Nix",
                    active: format == Some("nix"),
                    onclick: move |_| {
                        active_format.set(Some("nix"));
                        copied.set(false);
                    },
                }

                // Share link
                button {
                    class: "export-btn",
                    onclick: move |_| {
                        // Copy current URL to clipboard
                        let js = "navigator.clipboard.writeText(window.location.href)";
                        eval(js);
                        copied.set(true);
                    },
                    "Copy Link"
                }

                if is_copied {
                    span {
                        style: "font-size: 0.75rem; color: #a6e3a1;",
                        "Copied!"
                    }
                }
            }

            if let Some(text) = &content {
                div {
                    style: "position: relative;",

                    button {
                        class: "export-copy-btn",
                        onclick: {
                            let text = text.clone();
                            move |_| {
                                let escaped = text.replace('\\', "\\\\")
                                    .replace('`', "\\`")
                                    .replace('$', "\\$");
                                let js = format!("navigator.clipboard.writeText(`{escaped}`)");
                                eval(&js);
                                copied.set(true);
                            }
                        },
                        if is_copied { "Copied!" } else { "Copy" }
                    }

                    pre {
                        class: "mono",
                        style: "background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 0.5rem; \
                                overflow-x: auto; font-size: 0.8rem; border: 1px solid rgba(255,255,255,0.1);",
                        "{text}"
                    }
                }
            }
        }
    }
}

#[component]
fn ExportFormatBtn(
    label: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let style = if active {
        "background: rgba(122, 162, 247, 0.2); color: #7aa2f7; border-color: #7aa2f7;"
    } else {
        "background: transparent; color: inherit; border-color: rgba(255,255,255,0.15);"
    };

    rsx! {
        button {
            class: "export-btn",
            style: "{style}",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// Color diff table showing which colors differ between compared themes.
#[component]
fn ColorDiffTable(themes: Vec<litmus_model::Theme>) -> Element {
    let mut expanded = use_signal(|| false);
    let is_expanded = *expanded.read();

    // Build diff rows: (name, [hex per theme], differs?)
    let mut diff_rows: Vec<(String, Vec<String>, bool)> = Vec::new();

    // Helper to add a row
    let add_row = |rows: &mut Vec<(String, Vec<String>, bool)>,
                   name: &str,
                   values: Vec<String>| {
        let differs = values.windows(2).any(|w| w[0] != w[1]);
        rows.push((name.to_string(), values, differs));
    };

    // Special colors
    add_row(&mut diff_rows, "bg", themes.iter().map(|t| t.background.to_hex()).collect());
    add_row(&mut diff_rows, "fg", themes.iter().map(|t| t.foreground.to_hex()).collect());
    add_row(&mut diff_rows, "cursor", themes.iter().map(|t| t.cursor.to_hex()).collect());

    // ANSI colors
    for (i, name) in ANSI_NAMES.iter().enumerate() {
        let values: Vec<String> = themes
            .iter()
            .map(|t| t.ansi.as_array()[i].to_hex())
            .collect();
        add_row(&mut diff_rows, name, values);
    }

    let diff_count = diff_rows.iter().filter(|(_, _, d)| *d).count();

    rsx! {
        div {
            style: "margin-bottom: 1.5rem; border: 1px solid rgba(255,255,255,0.1); \
                    border-radius: 0.5rem; overflow: hidden;",

            // Header
            div {
                style: "padding: 0.5rem 0.75rem; cursor: pointer; display: flex; \
                        justify-content: space-between; align-items: center; \
                        background: rgba(255,255,255,0.03);",
                onclick: move |_| expanded.set(!is_expanded),

                span {
                    class: "mono",
                    style: "font-size: 0.8rem;",
                    "Color differences: {diff_count}/19"
                }
                span {
                    class: "mono",
                    style: "font-size: 0.7rem; opacity: 0.5;",
                    if is_expanded { "collapse" } else { "expand" }
                }
            }

            // Table
            if is_expanded {
                div {
                    style: "padding: 0.5rem 0.75rem; overflow-x: auto;",

                    table {
                        style: "width: 100%; border-collapse: collapse; font-size: 0.75rem; \
                                font-family: monospace;",

                        thead {
                            tr {
                                th {
                                    style: "text-align: left; padding: 0.25rem 0.5rem; \
                                            border-bottom: 1px solid rgba(255,255,255,0.1);",
                                    "Color"
                                }
                                for theme in &themes {
                                    th {
                                        style: "text-align: left; padding: 0.25rem 0.5rem; \
                                                border-bottom: 1px solid rgba(255,255,255,0.1);",
                                        "{theme.name}"
                                    }
                                }
                            }
                        }

                        tbody {
                            for (name, values, differs) in &diff_rows {
                                tr {
                                    style: if *differs {
                                        "background: rgba(247, 168, 184, 0.05);"
                                    } else {
                                        ""
                                    },
                                    td {
                                        style: "padding: 0.25rem 0.5rem; opacity: 0.8; \
                                                white-space: nowrap;",
                                        "{name}"
                                    }
                                    for val in values {
                                        td {
                                            style: "padding: 0.25rem 0.5rem;",
                                            div {
                                                style: "display: flex; align-items: center; gap: 0.3rem;",
                                                div {
                                                    style: "width: 14px; height: 14px; border-radius: 2px; \
                                                            background: {val}; border: 1px solid rgba(255,255,255,0.2); \
                                                            flex-shrink: 0;",
                                                }
                                                span {
                                                    style: "opacity: 0.7; font-size: 0.65rem;",
                                                    "{val}"
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

fn theme_slug(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}

/// Multi-theme comparison (2-4 themes side by side).
#[component]
fn CompareThemes(slugs: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();
    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).collect();

    let compare_themes: Vec<litmus_model::Theme> = slug_list
        .iter()
        .filter_map(|slug| all_themes.iter().find(|t| theme_slug(&t.name) == *slug).cloned())
        .collect();

    if compare_themes.is_empty() {
        return rsx! {
            div {
                h2 { "No themes found" }
                p { "Could not find any matching themes." }
                Link {
                    to: Route::ThemeList {},
                    style: "color: #7aa2f7;",
                    "Back to all themes"
                }
            }
        };
    }

    let n = compare_themes.len();
    let title = compare_themes
        .iter()
        .map(|t| t.name.as_str())
        .collect::<Vec<_>>()
        .join(" vs ");
    let grid_cols = format!("repeat({n}, 1fr)");

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
                style: "font-size: 1.3rem; margin-bottom: 1rem;",
                "{title}"
            }

            // Theme selectors
            CompareSelector {
                all_themes: all_themes.clone(),
                current_slugs: slug_list.iter().map(|s| s.to_string()).collect(),
            }

            // Color diff table
            if compare_themes.len() >= 2 {
                ColorDiffTable { themes: compare_themes.clone() }
            }

            for scene in &scenes {
                div {
                    style: "margin-bottom: 2rem;",

                    h3 {
                        style: "font-size: 0.95rem; margin-bottom: 0.75rem; opacity: 0.8;",
                        "{scene.name}"
                    }

                    div {
                        style: "display: grid; grid-template-columns: {grid_cols}; gap: 0.75rem; \
                                overflow-x: auto;",

                        for theme in &compare_themes {
                            div {
                                style: "min-width: 250px;",
                                div {
                                    style: "display: flex; justify-content: space-between; \
                                            align-items: center; margin-bottom: 0.25rem;",
                                    span {
                                        style: "font-size: 0.8rem; opacity: 0.7;",
                                        "{theme.name}"
                                    }
                                    Link {
                                        to: Route::ThemeDetail {
                                            slug: theme_slug(&theme.name),
                                        },
                                        style: "font-size: 0.7rem; color: #a6e3a1; text-decoration: none;",
                                        "Choose"
                                    }
                                }
                                scene_renderer::SceneView {
                                    theme: theme.clone(),
                                    scene: scene.clone(),
                                    compact: n > 2,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Dropdowns for selecting comparison themes (supports 2-4).
#[component]
fn CompareSelector(
    all_themes: Vec<litmus_model::Theme>,
    current_slugs: Vec<String>,
) -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            style: "display: flex; gap: 0.5rem; margin-bottom: 1.5rem; flex-wrap: wrap; \
                    align-items: center;",

            for (idx, slug) in current_slugs.iter().enumerate() {
                {
                    let all = all_themes.clone();
                    let slugs = current_slugs.clone();
                    let current_val = slug.clone();
                    rsx! {
                        if idx > 0 {
                            span { style: "opacity: 0.6; font-size: 0.85rem;", "vs" }
                        }
                        select {
                            style: "background: #1a1b26; color: #c0caf5; border: 1px solid #33467c; \
                                    padding: 0.4rem 0.6rem; border-radius: 0.25rem; font-size: 0.85rem;",
                            value: "{current_val}",
                            onchange: move |evt: Event<FormData>| {
                                let mut new_slugs = slugs.clone();
                                new_slugs[idx] = evt.value();
                                nav.push(Route::CompareThemes {
                                    slugs: new_slugs.join(","),
                                });
                            },
                            for t in &all {
                                option {
                                    value: "{theme_slug(&t.name)}",
                                    "{t.name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Scene-centric view: one scene rendered across all themes in a grid.
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
                        style: "font-size: 0.85rem; opacity: 0.7; margin-bottom: 1rem;",
                        "{scene.description}"
                    }

                    // Scene selector tabs
                    div { class: "scene-tabs",
                        style: "margin-bottom: 1.5rem;",
                        for s in &scenes {
                            Link {
                                to: Route::SceneAcrossThemes { scene_id: s.id.clone() },
                                class: if s.id == scene.id { "scene-tab scene-tab-active" } else { "scene-tab" },
                                style: "text-decoration: none;",
                                "{s.name}"
                            }
                        }
                    }

                    // Grid of all themes with compact scene rendering
                    div { class: "scene-grid",
                        for theme in &all_themes {
                            div { class: "scene-grid-card",
                                div {
                                    style: "display: flex; justify-content: space-between; \
                                            align-items: center; margin-bottom: 0.25rem;",
                                    Link {
                                        to: Route::ThemeDetail {
                                            slug: theme_slug(&theme.name),
                                        },
                                        style: "color: #7aa2f7; text-decoration: none; \
                                                font-size: 0.8rem; font-weight: bold;",
                                        "{theme.name}"
                                    }
                                    CompareToggle {
                                        slug: theme_slug(&theme.name),
                                        name: theme.name.clone(),
                                    }
                                }
                                scene_renderer::SceneView {
                                    theme: theme.clone(),
                                    scene: scene.clone(),
                                    compact: true,
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
