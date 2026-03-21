use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page with scene navigation via sidebar's ActiveScene.
#[component]
pub fn ThemeDetail(slug: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let theme = all_themes.iter().find(|t| theme_slug(&t.name) == slug);
    let mut palette_expanded = use_signal(|| false);
    let mut issues_expanded = use_signal(|| false);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let active_scene = use_context::<Signal<ActiveScene>>();

    match theme {
        Some(theme) => {
            let cvd = cvd_sim.read().0;
            let base_theme = theme.clone();
            let theme = maybe_simulate(&base_theme, cvd);
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();
            let this_slug = theme_slug(&theme.name);
            let scenes = litmus_model::scenes::all_scenes();
            let scene_idx = active_scene.read().0.unwrap_or(0);
            let tab_idx = scene_idx.min(scenes.len().saturating_sub(1));
            let expanded = *palette_expanded.read();
            let issues_open = *issues_expanded.read();

            let issues = litmus_model::contrast::validate_theme_readability(&theme);
            let issue_count = issues.len();
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );
            let readability = litmus_model::contrast::readability_score(&theme) as u8;

            let scene_count = scenes.len();
            let mut shortlist = use_context::<Signal<Shortlist>>();
            let detail_slug = this_slug.clone();
            let mut active_scene_write = active_scene;

            // Count issues per scene for tab badges
            let mut issues_per_scene: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
            for issue in &issues {
                *issues_per_scene.entry(issue.scene_id.as_str()).or_insert(0) += 1;
            }

            // Group issues by scene, deduplicated by slug (same color pair = same issue type)
            let mut issues_by_scene: Vec<(String, Vec<&litmus_model::contrast::ContrastIssue>)> = Vec::new();
            let mut seen_slugs: std::collections::HashSet<&str> = std::collections::HashSet::new();
            for issue in &issues {
                if !seen_slugs.insert(issue.slug.as_str()) {
                    continue;
                }
                if let Some(group) = issues_by_scene.iter_mut().find(|(id, _)| id == &issue.scene_id) {
                    group.1.push(issue);
                } else {
                    issues_by_scene.push((issue.scene_id.clone(), vec![issue]));
                }
            }

            rsx! {
                div {
                    class: "page-theme-detail",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: move |evt: Event<KeyboardData>| {
                        match evt.key() {
                            Key::ArrowLeft => {
                                if tab_idx > 0 {
                                    active_scene_write.set(ActiveScene(Some(tab_idx - 1)));
                                }
                            }
                            Key::ArrowRight => {
                                if tab_idx + 1 < scene_count {
                                    active_scene_write.set(ActiveScene(Some(tab_idx + 1)));
                                }
                            }
                            Key::Character(ref c) if c == "c" => {
                                let mut sel = shortlist.write();
                                if let Some(pos) = sel.0.iter().position(|s| s == &detail_slug) {
                                    sel.0.remove(pos);
                                } else if sel.0.len() < MAX_SHORTLIST {
                                    sel.0.push(detail_slug.clone());
                                }
                            }
                            _ => {}
                        }
                    },

                    // Theme header with inline metadata
                    div { class: "detail-header",
                        h2 { class: "page-title", "{theme.name}" }
                        span { class: "mono detail-ratio",
                            if fg_bg_ratio >= litmus_model::contrast::WCAG_AA_NORMAL {
                                span { class: "text-success", "{fg_bg_ratio:.1}:1" }
                            } else {
                                span { class: "text-error", "{fg_bg_ratio:.1}:1" }
                            }
                        }
                        span { class: "detail-readability mono", "readability: {readability}%" }
                        if issue_count > 0 {
                            button {
                                class: "detail-issues-toggle text-error",
                                onclick: move |_| issues_expanded.set(!issues_open),
                                if issues_open {
                                    "{issue_count} contrast issue(s) \u{25BC}"
                                } else {
                                    "{issue_count} contrast issue(s) \u{25B6}"
                                }
                            }
                        }
                        ShortlistToggle { slug: this_slug.clone(), name: theme.name.clone() }
                        UseAsAppThemeButton { slug: this_slug }
                    }

                    // Expandable contrast issues
                    if issues_open && issue_count > 0 {
                        div { class: "contrast-issues-list",
                            for (scene_id, scene_issues) in &issues_by_scene {
                                {
                                    let target_idx = scenes.iter().position(|s| s.id == *scene_id).unwrap_or(0);
                                    rsx! {
                                        div { class: "contrast-issue-group",
                                            button {
                                                class: "contrast-issue-scene mono",
                                                onclick: move |_| active_scene_write.set(ActiveScene(Some(target_idx))),
                                                "{scene_id} \u{2192}"
                                            }
                                            for issue in scene_issues.iter() {
                                                div { class: "contrast-issue-item",
                                                    span {
                                                        class: "contrast-issue-sample mono",
                                                        style: "color: {issue.fg.to_hex()}; background: {issue.bg.to_hex()};",
                                                        "Sample text"
                                                    }
                                                    span { class: "contrast-issue-ratio mono",
                                                        "{issue.ratio:.1}:1"
                                                        span { class: "contrast-issue-need", " (need {issue.threshold:.1}:1)" }
                                                    }
                                                    span { class: "contrast-issue-hex mono",
                                                        span {
                                                            class: "color-chip",
                                                            style: "background: {issue.fg.to_hex()};",
                                                        }
                                                        " {issue.fg.to_hex()} / "
                                                        span {
                                                            class: "color-chip",
                                                            style: "background: {issue.bg.to_hex()};",
                                                        }
                                                        " {issue.bg.to_hex()}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Scene tabs (at top, before palette)
                    div { class: "scene-nav",
                        div { class: "scene-tabs", role: "tablist",
                            for (i, scene) in scenes.iter().enumerate() {
                                {
                                    let scene_issue_count = issues_per_scene.get(scene.id.as_str()).copied().unwrap_or(0);
                                    rsx! {
                                        button {
                                            class: if i == tab_idx { "scene-tab scene-tab-active" } else { "scene-tab" },
                                            role: "tab",
                                            aria_selected: if i == tab_idx { "true" } else { "false" },
                                            onclick: move |_| active_scene_write.set(ActiveScene(Some(i))),
                                            "{scene.name}"
                                            if scene_issue_count > 0 {
                                                span { class: "scene-tab-badge", "{scene_issue_count}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        span { class: "mono scene-hint", "\u{2190} \u{2192} navigate \u{00B7} c shortlist" }
                    }

                    // Active scene
                    if let Some(scene) = scenes.get(tab_idx) {
                        {
                            let current_issue_spans: Vec<(usize, usize)> = issues.iter()
                                .filter(|i| i.scene_id == scene.id)
                                .map(|i| (i.line, i.span))
                                .collect();
                            rsx! {
                                div { role: "tabpanel",
                                    scene_renderer::SceneView {
                                        theme: theme.clone(),
                                        scene: scene.clone(),
                                        issue_spans: current_issue_spans,
                                    }
                                }
                            }
                        }
                    }

                    // Compact color palette (expandable)
                    div {
                        class: "color-palette",
                        style: "background: {bg}; color: {fg};",

                        div {
                            class: "palette-compact",
                            onclick: move |_| palette_expanded.set(!expanded),

                            ColorSwatch { label: "bg", color: theme.background.to_hex() }
                            ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                            ColorSwatch { label: "cur", color: theme.cursor.to_hex() }

                            span { class: "palette-divider", "|" }

                            div { class: "swatch-row",
                                for color in theme.ansi.as_array().iter() {
                                    div {
                                        class: "swatch",
                                        style: "background: {color.to_hex()};",
                                        title: "{color.to_hex()}",
                                    }
                                }
                            }

                            span { class: "mono palette-toggle",
                                if expanded { "collapse" } else { "expand" }
                            }
                        }

                        if expanded {
                            div { class: "palette-expanded",
                                div { class: "special-colors",
                                    ColorSwatch { label: "bg", color: theme.background.to_hex() }
                                    ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                                    ColorSwatch { label: "cursor", color: theme.cursor.to_hex() }
                                    ColorSwatch { label: "sel bg", color: theme.selection_background.to_hex() }
                                    ColorSwatch { label: "sel fg", color: theme.selection_foreground.to_hex() }
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

                    ExportButtons { theme: theme.clone() }
                }
            }
        }
        None => {
            rsx! {
                div {
                    h2 { "Theme not found" }
                    p { "No theme matches \"{slug}\"." }
                    Link { to: Route::ThemeList {}, class: "accent-link", "Back to all themes" }
                }
            }
        }
    }
}
