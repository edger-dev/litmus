use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

use crate::components::{ColorSwatch, ScoreRing};
use crate::fixtures;
use crate::screenshot_view::{has_screenshot_for_provider, ScreenshotImage};
use crate::state::*;
use crate::term_renderer::{self, build_issue_registry, SpanIssueDetail};
use crate::themes;
use crate::Route;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Pre-computed contrast data for a single theme.
struct ThemeContrastData {
    readability: u8,
    issue_count: usize,
    /// Per-fixture issue details: fixture_id → Vec<(line, span, detail)>
    issues_per_fixture: HashMap<String, Vec<(usize, usize, SpanIssueDetail)>>,
}

fn compute_theme_contrast(
    theme: &litmus_model::Theme,
    all_fixtures: &[litmus_model::term_output::TermOutput],
) -> ThemeContrastData {
    let issues = litmus_model::contrast::validate_fixtures_contrast(all_fixtures, theme);
    let (_rules, id_map) = build_issue_registry(&issues);
    let readability =
        litmus_model::contrast::term_readability_score(theme, all_fixtures) as u8;

    let mut issues_per_fixture: HashMap<String, Vec<(usize, usize, SpanIssueDetail)>> =
        HashMap::new();
    for issue in &issues {
        let rule_id = id_map.get(&(issue.fg_term, issue.bg_term)).cloned();
        issues_per_fixture
            .entry(issue.fixture_id.clone())
            .or_default()
            .push((
                issue.line,
                issue.span,
                SpanIssueDetail {
                    rule_id,
                    ratio: issue.ratio,
                    threshold: issue.threshold,
                    fg_hex: issue.fg.to_hex(),
                    bg_hex: issue.bg.to_hex(),
                },
            ));
    }

    let issue_count = {
        let unique: HashSet<Option<&str>> = issues_per_fixture
            .values()
            .flat_map(|v| v.iter().map(|(_, _, d)| d.rule_id.as_deref()))
            .collect();
        unique.len()
    };

    ThemeContrastData {
        readability,
        issue_count,
        issues_per_fixture,
    }
}

/// Count unique contrast rules in a fixture's issue list.
fn unique_issue_count(issues: &[(usize, usize, SpanIssueDetail)]) -> usize {
    let unique: HashSet<Option<&str>> = issues
        .iter()
        .map(|(_, _, d)| d.rule_id.as_deref())
        .collect();
    unique.len()
}

/// Multi-theme comparison (2-4 themes side by side).
#[component]
pub fn CompareThemes(provider: String, slugs: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let provider = active_provider.read().0.clone();
    let all_themes = themes::themes_for_provider(&provider);
    let all_fixtures = fixtures::all_fixtures();
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;
    let manifest_state = use_context::<Signal<ManifestState>>();
    let mut show_screenshots = use_signal(|| false);

    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).collect();

    let compare_themes: Vec<litmus_model::Theme> = slug_list
        .iter()
        .filter_map(|slug| all_themes.iter().find(|t| theme_slug(&t.name) == *slug).cloned())
        .map(|t| maybe_simulate(&t, cvd))
        .collect();

    if compare_themes.is_empty() {
        return rsx! {
            div {
                h2 { "No themes found" }
                p { "Could not find any matching themes." }
                Link { to: Route::ThemeList { provider: provider.clone() }, class: "accent-link", "Back to all themes" }
            }
        };
    }

    // Compute contrast data for each theme
    let contrast_data: Vec<ThemeContrastData> = compare_themes
        .iter()
        .map(|theme| compute_theme_contrast(theme, all_fixtures))
        .collect();

    let n = compare_themes.len();
    let grid_cols = format!("repeat({n}, 1fr)");
    let screenshots_on = *show_screenshots.read();

    rsx! {
        div { class: "page-compare",
            style: "--compare-cols: {grid_cols};",

            // Toolbar: toggle + column headers
            div { class: "compare-toolbar",
                div { class: "compare-view-toggle",
                    button {
                        class: if !screenshots_on { "compare-toggle-btn compare-toggle-btn-active" } else { "compare-toggle-btn" },
                        onclick: move |_| show_screenshots.set(false),
                        "Simulated"
                    }
                    button {
                        class: if screenshots_on { "compare-toggle-btn compare-toggle-btn-active" } else { "compare-toggle-btn" },
                        onclick: move |_| show_screenshots.set(true),
                        "Screenshot"
                    }
                }
            }

            // Sticky column headers with theme names + readability
            div { class: "compare-column-headers",
                for (theme, cdata) in compare_themes.iter().zip(contrast_data.iter()) {
                    div { class: "compare-column-header",
                        Link {
                            to: Route::ThemeDetail {
                                provider: provider.clone(),
                                slug: theme_slug(&theme.name),
                            },
                            class: "compare-column-header-link",
                            "{theme.name}"
                        }
                        div { class: "compare-column-meta",
                            ScoreRing { score: cdata.readability, size: 22.0 }
                            span { class: "mono compare-readability", "{cdata.readability}%" }
                            if cdata.issue_count > 0 {
                                span { class: "compare-issue-count text-error",
                                    "{cdata.issue_count}"
                                }
                            }
                        }
                    }
                }
            }

            for fixture in all_fixtures {
                {
                    // Compute per-theme issue counts for this fixture's header badge
                    let fixture_issue_counts: Vec<usize> = contrast_data.iter().map(|cd| {
                        cd.issues_per_fixture.get(&fixture.id)
                            .map(|v| unique_issue_count(v))
                            .unwrap_or(0)
                    }).collect();
                    let max_fixture_issues = fixture_issue_counts.iter().copied().max().unwrap_or(0);

                    rsx! {
                        div { class: "compare-scene-group",
                            id: "scene-{fixture.id}",
                            h3 { class: "compare-scene-name",
                                "{fixture.name}"
                                if max_fixture_issues > 0 {
                                    span { class: "scene-tab-badge", "{max_fixture_issues}" }
                                }
                            }

                            div { class: "compare-grid",
                                for (theme, cdata) in compare_themes.iter().zip(contrast_data.iter()) {
                                    div { class: "compare-grid-item",
                                        if screenshots_on {
                                            {
                                                let t_slug = theme_slug(&theme.name);
                                                let has_screenshot = has_screenshot_for_provider(
                                                    &manifest_state.read().0,
                                                    &provider,
                                                    &t_slug,
                                                    &fixture.id,
                                                );
                                                if has_screenshot {
                                                    rsx! {
                                                        ScreenshotImage {
                                                            theme_slug: t_slug,
                                                            fixture_id: fixture.id.clone(),
                                                            provider: provider.clone(),
                                                        }
                                                    }
                                                } else {
                                                    rsx! {
                                                        div { class: "compare-screenshot-placeholder",
                                                            "No screenshot"
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            {
                                                let fixture_issues = cdata.issues_per_fixture
                                                    .get(&fixture.id)
                                                    .cloned()
                                                    .unwrap_or_default();
                                                rsx! {
                                                    term_renderer::TermOutputView {
                                                        theme: theme.clone(),
                                                        output: fixture.clone(),
                                                        compact: n > 2,
                                                        issue_details: fixture_issues,
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

            // Color palettes at the end
            h3 { class: "compare-scene-name", "Color Palette" }
            div { class: "compare-grid",
                for theme in &compare_themes {
                    {
                        let bg = theme.background.to_hex();
                        let fg = theme.foreground.to_hex();
                        rsx! {
                            div { class: "compare-grid-item",
                                div {
                                    class: "color-palette",
                                    style: "background: {bg}; color: {fg};",
                                    div { class: "palette-expanded",
                                        div { class: "special-colors",
                                            ColorSwatch { label: "bg".to_string(), color: theme.background.to_hex() }
                                            ColorSwatch { label: "fg".to_string(), color: theme.foreground.to_hex() }
                                            ColorSwatch { label: "cursor".to_string(), color: theme.cursor.to_hex() }
                                            ColorSwatch { label: "sel bg".to_string(), color: theme.selection_background.to_hex() }
                                            ColorSwatch { label: "sel fg".to_string(), color: theme.selection_foreground.to_hex() }
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
                        }
                    }
                }
            }
        }
    }
}
