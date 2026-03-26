use dioxus::prelude::*;

use crate::components::*;
use crate::fixtures;
use crate::screenshot_view::{has_screenshot_for_provider, ScreenshotImage};
use crate::state::*;
use crate::term_renderer::{self, SpanIssueDetail};
use crate::themes;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page — all scenes rendered vertically.
#[component]
pub fn ThemeDetail(slug: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    // Only shows themes available for the active provider. Themes unavailable for
    // the current provider show "not found" — the browse page blocks navigation to
    // them, but direct URL access is possible.
    let all_themes = themes::themes_for_provider(&active_provider.read().0);
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
            let all_fixtures = fixtures::all_fixtures();
            let expanded = *palette_expanded.read();

            let issues = litmus_model::contrast::validate_fixtures_contrast(all_fixtures, &theme);

            // Deduplicate issues by unique (fg, bg) color pair
            let mut seen = std::collections::HashSet::new();
            let mut unique_issue_list: Vec<(String, String, f64)> = Vec::new();
            for i in &issues {
                let key = (i.fg.to_hex(), i.bg.to_hex());
                if seen.insert(key.clone()) {
                    unique_issue_list.push((key.0, key.1, i.ratio));
                }
            }
            let issue_count = unique_issue_list.len();
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );
            let readability = litmus_model::contrast::term_readability_score(&theme, all_fixtures) as u8;

            let mut shortlist = use_context::<Signal<Shortlist>>();
            let app_theme = use_context::<Signal<AppThemeSlug>>();
            let is_current_theme = app_theme.read().0.as_deref() == Some(this_slug.as_str());
            let detail_slug = this_slug.clone();
            let manifest_state = use_context::<Signal<ManifestState>>();
            let cur_provider = active_provider.read().0.clone();

            // Group issues per fixture as (line, span, detail) tuples
            let mut issues_per_fixture: std::collections::HashMap<&str, Vec<(usize, usize, SpanIssueDetail)>> = std::collections::HashMap::new();
            for issue in &issues {
                issues_per_fixture.entry(issue.fixture_id.as_str()).or_default().push(
                    (issue.line, issue.span, SpanIssueDetail {
                        ratio: issue.ratio,
                        threshold: issue.threshold,
                        fg_hex: issue.fg.to_hex(),
                        bg_hex: issue.bg.to_hex(),
                    })
                );
            }

            // Publish per-fixture unique issue counts to context for the minimap
            let mut scene_issue_counts = use_context::<Signal<SceneIssueCounts>>();
            let counts: std::collections::HashMap<String, usize> = issues_per_fixture
                .iter()
                .map(|(k, v)| {
                    let unique: std::collections::HashSet<(&str, &str)> = v
                        .iter()
                        .map(|(_, _, d)| (d.fg_hex.as_str(), d.bg_hex.as_str()))
                        .collect();
                    (k.to_string(), unique.len())
                })
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
                        UseAsAppThemeButton { slug: this_slug.clone() }

                        if issue_count > 0 {
                            div { class: "detail-issues-list",
                                for (fg_hex, bg_hex, ratio) in &unique_issue_list {
                                    span { class: "detail-issue-chip",
                                        span {
                                            class: "color-chip",
                                            style: "background: {fg_hex};",
                                        }
                                        " on "
                                        span {
                                            class: "color-chip",
                                            style: "background: {bg_hex};",
                                        }
                                        span { class: "detail-issue-chip-ratio", " {ratio:.1}:1" }
                                    }
                                }
                            }
                        }
                    }

                    // All fixtures rendered as side-by-side (terminal output + screenshot)
                    for fixture in all_fixtures {
                        {
                            let fixture_issues = issues_per_fixture
                                .get(fixture.id.as_str())
                                .cloned()
                                .unwrap_or_default();
                            let fixture_issue_count = {
                                let unique: std::collections::HashSet<(&str, &str)> = fixture_issues
                                    .iter()
                                    .map(|(_, _, d)| (d.fg_hex.as_str(), d.bg_hex.as_str()))
                                    .collect();
                                unique.len()
                            };
                            let has_screenshot = has_screenshot_for_provider(
                                &manifest_state.read().0,
                                &cur_provider,
                                &this_slug,
                                &fixture.id,
                            );
                            rsx! {
                                div {
                                    class: "detail-scene-section",
                                    id: "scene-{fixture.id}",
                                    h3 { class: "detail-scene-heading",
                                        "{fixture.name}"
                                        if fixture_issue_count > 0 {
                                            span { class: "scene-tab-badge", "{fixture_issue_count}" }
                                        }
                                    }
                                    div { class: "scene-split",
                                        // Left: rendered terminal output
                                        div { class: "scene-split-panel",
                                            span { class: "scene-split-label", "Terminal Output" }
                                            term_renderer::TermOutputView {
                                                theme: theme.clone(),
                                                output: fixture.clone(),
                                                issue_details: fixture_issues,
                                            }
                                        }
                                        // Right: real screenshot or placeholder
                                        div { class: "scene-split-panel",
                                            span { class: "scene-split-label", "Screenshot" }
                                            if has_screenshot {
                                                ScreenshotImage {
                                                    theme_slug: this_slug.clone(),
                                                    fixture_id: fixture.id.clone(),
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
