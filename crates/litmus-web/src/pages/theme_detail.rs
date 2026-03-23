use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer::{self, SpanIssueDetail};
use crate::screenshot_view::{scene_id_to_fixture_id, ProviderSelector, ScreenshotSceneView};
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
            let mut active_provider = use_context::<Signal<ActiveProvider>>();
            let current_provider = active_provider.read().0.clone();

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
                    }

                    // Provider selector (shown when screenshots are available)
                    ProviderSelector {
                        theme_slug: theme_slug(&theme.name),
                        active: current_provider.clone(),
                        on_change: move |slug: String| active_provider.set(ActiveProvider(slug)),
                    }

                    // All scenes rendered vertically
                    for scene in &scenes {
                        {
                            let scene_issues = issues_per_scene
                                .get(scene.id.as_str())
                                .cloned()
                                .unwrap_or_default();
                            let scene_issue_count = scene_issues.len();
                            let fixture_id = scene_id_to_fixture_id(&scene.id);
                            let use_screenshot = !current_provider.starts_with("simulated")
                                && fixture_id.is_some();
                            let provider_slug = current_provider.clone();
                            let t_slug = theme_slug(&theme.name);
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
                                    if use_screenshot {
                                        ScreenshotSceneView {
                                            provider: provider_slug,
                                            theme_slug: t_slug,
                                            fixture_id: fixture_id.unwrap().to_string(),
                                            fallback_theme: theme.clone(),
                                            fallback_scene: scene.clone(),
                                        }
                                    } else {
                                        scene_renderer::SceneView {
                                            theme: theme.clone(),
                                            scene: scene.clone(),
                                            issue_details: scene_issues,
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
