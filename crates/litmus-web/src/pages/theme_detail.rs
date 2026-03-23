use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer::{self, SpanIssueDetail};
use crate::screenshot_view::{
    has_screenshot_for_provider, manifest_provider_slugs, scene_id_to_fixture_id, ScreenshotImage,
};
use crate::state::*;
use crate::themes;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page — all scenes rendered vertically.
#[component]
pub fn ThemeDetail(slug: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let theme = all_themes.iter().find(|t| theme_slug(&t.name) == slug);
    let mut palette_expanded = use_signal(|| true);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();

    match theme {
        Some(theme) => {
            let cvd = cvd_sim.read().0;
            let base_theme = theme.clone();
            let theme = maybe_simulate(&base_theme, cvd);
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();
            let this_slug = theme_slug(&theme.name);
            let scenes = litmus_model::scenes::all_scenes();
            let expanded = *palette_expanded.read();

            let issues = litmus_model::contrast::validate_theme_readability(&theme);
            let issue_count = issues.len();
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );
            let readability = litmus_model::contrast::readability_score(&theme) as u8;

            let mut shortlist = use_context::<Signal<Shortlist>>();
            let app_theme = use_context::<Signal<AppThemeSlug>>();
            let is_current_theme = app_theme.read().0.as_deref() == Some(this_slug.as_str());
            let detail_slug = this_slug.clone();
            let manifest_state = use_context::<Signal<ManifestState>>();
            let providers = manifest_provider_slugs(&manifest_state.read().0);
            let mut selected_provider = use_signal(|| "kitty".to_string());

            // Group issues per scene as (line, span, detail) tuples
            let mut issues_per_scene: std::collections::HashMap<&str, Vec<(usize, usize, SpanIssueDetail)>> = std::collections::HashMap::new();
            for issue in &issues {
                issues_per_scene.entry(issue.scene_id.as_str()).or_default().push(
                    (issue.line, issue.span, SpanIssueDetail {
                        ratio: issue.ratio,
                        threshold: issue.threshold,
                        level: issue.level.to_string(),
                        fg_hex: issue.fg.to_hex(),
                        bg_hex: issue.bg.to_hex(),
                    })
                );
            }

            // Publish per-scene issue counts to context for the minimap
            let mut scene_issue_counts = use_context::<Signal<SceneIssueCounts>>();
            let counts: std::collections::HashMap<String, usize> = issues_per_scene
                .iter()
                .map(|(k, v)| (k.to_string(), v.len()))
                .collect();
            if counts != scene_issue_counts.read().0 {
                scene_issue_counts.set(SceneIssueCounts(counts));
            }

            rsx! {
                div {
                    class: "page-theme-detail",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: move |evt: Event<KeyboardData>| {
                        match evt.key() {
                            Key::Character(ref c) if c == "c" => {
                                if !is_current_theme {
                                    let mut sel = shortlist.write();
                                    if let Some(pos) = sel.0.iter().position(|s| s == &detail_slug) {
                                        sel.0.remove(pos);
                                    } else {
                                        if sel.0.len() >= MAX_SHORTLIST {
                                            sel.0.remove(0);
                                        }
                                        sel.0.push(detail_slug.clone());
                                    }
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
                            span { class: "detail-issues-count text-error",
                                "{issue_count} contrast issue(s)"
                            }
                        }
                        ShortlistCheckbox { slug: this_slug.clone(), name: theme.name.clone() }
                        UseAsAppThemeButton { slug: this_slug }
                        if !providers.is_empty() {
                            div { class: "detail-provider-select",
                                label { class: "detail-provider-label", "Provider:" }
                                select {
                                    value: "{selected_provider.read()}",
                                    onchange: move |evt: Event<FormData>| {
                                        selected_provider.set(evt.value());
                                    },
                                    for slug in &providers {
                                        option {
                                            value: "{slug}",
                                            selected: *selected_provider.read() == *slug,
                                            "{slug}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // All scenes rendered as side-by-side (simulated + screenshot)
                    for scene in &scenes {
                        {
                            let scene_issues = issues_per_scene
                                .get(scene.id.as_str())
                                .cloned()
                                .unwrap_or_default();
                            let scene_issue_count = scene_issues.len();
                            let fixture_id = scene_id_to_fixture_id(&scene.id);
                            let t_slug = theme_slug(&theme.name);
                            let cur_provider = selected_provider.read().clone();
                            let has_screenshot = fixture_id.is_some()
                                && has_screenshot_for_provider(
                                    &manifest_state.read().0,
                                    &cur_provider,
                                    &t_slug,
                                    fixture_id.unwrap(),
                                );
                            rsx! {
                                div {
                                    class: "detail-scene-section",
                                    id: "scene-{scene.id}",
                                    h3 { class: "detail-scene-heading",
                                        "{scene.name}"
                                        if scene_issue_count > 0 {
                                            span { class: "scene-tab-badge", "{scene_issue_count}" }
                                        }
                                    }
                                    div { class: "scene-split",
                                        // Left: simulated rendering with contrast analysis
                                        div { class: "scene-split-panel",
                                            span { class: "scene-split-label", "Simulated" }
                                            scene_renderer::SceneView {
                                                theme: theme.clone(),
                                                scene: scene.clone(),
                                                issue_details: scene_issues,
                                            }
                                        }
                                        // Right: real screenshot or placeholder
                                        div { class: "scene-split-panel",
                                            span { class: "scene-split-label", "Screenshot" }
                                            if has_screenshot {
                                                ScreenshotImage {
                                                    theme_slug: t_slug,
                                                    fixture_id: fixture_id.unwrap().to_string(),
                                                    provider: cur_provider.clone(),
                                                }
                                            } else {
                                                div { class: "scene-split-placeholder",
                                                    "No screenshot captured"
                                                }
                                            }
                                        }
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
                    Link { to: crate::Route::ThemeList {}, class: "accent-link", "Back to all themes" }
                }
            }
        }
    }
}
