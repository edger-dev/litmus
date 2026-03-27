use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;

use crate::components::{ColorSwatch, ScoreRing};
use crate::fixtures;
use crate::screenshot_view::{has_screenshot_for_provider, ScreenshotImage};
use crate::state::*;
use crate::term_renderer::{self, build_issue_registry, ContrastRule, SpanIssueDetail};
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
    /// Deduplicated contrast rules for chip rendering.
    rules: Vec<ContrastRule>,
    /// Per-rule list of fixture IDs (in fixture order) for click-to-cycle.
    fixtures_per_rule: HashMap<String, Vec<String>>,
    /// Per-fixture issue details: fixture_id → Vec<(line, span, detail)>
    issues_per_fixture: HashMap<String, Vec<(usize, usize, SpanIssueDetail)>>,
}

fn compute_theme_contrast(
    theme: &litmus_model::Theme,
    all_fixtures: &[litmus_model::term_output::TermOutput],
) -> ThemeContrastData {
    let issues = litmus_model::contrast::validate_fixtures_contrast(all_fixtures, theme);
    let (rules, id_map) = build_issue_registry(&issues);
    let readability =
        litmus_model::contrast::term_readability_score(theme, all_fixtures) as u8;
    let issue_count = rules.len();

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

    // Build per-rule list of fixture IDs for cycling (preserves fixture order)
    let fixtures_per_rule: HashMap<String, Vec<String>> = {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for fixture in all_fixtures {
            if let Some(fixture_issues) = issues_per_fixture.get(&fixture.id) {
                let rule_ids: HashSet<&str> = fixture_issues
                    .iter()
                    .filter_map(|(_, _, d)| d.rule_id.as_deref())
                    .collect();
                for rid in rule_ids {
                    map.entry(rid.to_string()).or_default().push(fixture.id.clone());
                }
            }
        }
        map
    };

    ThemeContrastData {
        readability,
        issue_count,
        rules,
        fixtures_per_rule,
        issues_per_fixture,
    }
}

/// Count unique contrast rules in a fixture's issue list (excludes issues without rule IDs).
fn unique_issue_count(issues: &[(usize, usize, SpanIssueDetail)]) -> usize {
    let unique: HashSet<&str> = issues
        .iter()
        .filter_map(|(_, _, d)| d.rule_id.as_deref())
        .collect();
    unique.len()
}

/// Side-by-side comparison of two themes.
#[component]
pub fn CompareThemes(provider: String, slugs: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let provider = active_provider.read().0.clone();
    let all_themes = themes::themes_for_provider(&provider);
    let all_fixtures = fixtures::all_fixtures();
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;
    let manifest_state = use_context::<Signal<ManifestState>>();
    let nav = navigator();
    let mut show_screenshots = use_signal(|| false);
    // Active issue state: (theme_index, rule_id, fixture_cycle_index)
    let mut active_issue: Signal<Option<(usize, String, usize)>> = use_signal(|| None);

    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).take(MAX_COMPARE).collect();

    let compare_themes: Vec<litmus_model::Theme> = slug_list
        .iter()
        .filter_map(|slug| all_themes.iter().find(|t| theme_slug(&t.name) == *slug).cloned())
        .map(|t| maybe_simulate(&t, cvd))
        .collect();

    // Record the non-app-theme slug as the last compared theme
    // and push all compared themes into visit history
    {
        let mut last_compared = use_context::<Signal<LastComparedSlug>>();
        let mut visit_history = use_context::<Signal<VisitHistory>>();
        let app_theme = use_context::<Signal<AppThemeSlug>>();
        let app_s = app_theme.read().0.clone();
        // Pick the slug that isn't the app theme (the "other" side)
        let other = slug_list.iter().find(|s| app_s.as_deref() != Some(*s)).or(slug_list.first());
        if let Some(other_slug) = other {
            let other_str = other_slug.to_string();
            if last_compared.read().0.as_deref() != Some(other_str.as_str()) {
                last_compared.set(LastComparedSlug(Some(other_str)));
            }
        }
        // Push all compared slugs into history (most recent first, dedup)
        let mut h = visit_history.write();
        for slug in slug_list.iter().rev() {
            let s = slug.to_string();
            h.0.retain(|x| x != &s);
            h.0.insert(0, s);
        }
        h.0.truncate(MAX_HISTORY);
    }

    if compare_themes.is_empty() {
        return rsx! {
            div {
                h2 { "No themes found" }
                p { "Could not find any matching themes." }
                Link { to: Route::ThemeList { provider: provider.clone() }, class: "accent-link", "Back to all themes" }
            }
        };
    }

    // Build sorted theme list for the picker dropdowns
    let mut picker_themes: Vec<(String, String)> = all_themes
        .iter()
        .map(|t| (theme_slug(&t.name), t.name.clone()))
        .collect();
    picker_themes.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));

    // Compute contrast data for each theme
    let contrast_data: Vec<ThemeContrastData> = compare_themes
        .iter()
        .map(|theme| compute_theme_contrast(theme, all_fixtures))
        .collect();

    // Publish per-theme issue dots to context for the sidebar minimap
    let mut compare_dots = use_context::<Signal<CompareIssueDots>>();
    let dots: HashMap<String, Vec<(String, String, usize)>> = {
        let mut map: HashMap<String, Vec<(String, String, usize)>> = HashMap::new();
        for fixture in all_fixtures {
            let mut entries = Vec::new();
            for (theme, cdata) in compare_themes.iter().zip(contrast_data.iter()) {
                let count = cdata.issues_per_fixture
                    .get(&fixture.id)
                    .map(|v| unique_issue_count(v))
                    .unwrap_or(0);
                entries.push((theme.name.clone(), theme.foreground.to_hex(), count));
            }
            map.insert(fixture.id.clone(), entries);
        }
        map
    };
    let new_dots = CompareIssueDots(dots);
    if *compare_dots.read() != new_dots {
        compare_dots.set(new_dots);
    }

    let screenshots_on = *show_screenshots.read();
    let focused: Option<(usize, String)> = active_issue
        .read()
        .as_ref()
        .map(|(ti, rid, _)| (*ti, rid.clone()));

    rsx! {
        div { class: "page-compare",
            tabindex: "0",
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Escape {
                    active_issue.set(None);
                }
            },

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

            // Sticky column headers with theme picker + readability + issue chips
            div { class: "compare-column-headers",
                for (theme_idx, (theme, cdata)) in compare_themes.iter().zip(contrast_data.iter()).enumerate() {
                    {
                        let current_slug = theme_slug(&theme.name);
                        let picker_opts = picker_themes.clone();
                        let other_slug = if theme_idx == 0 {
                            slug_list.get(1).unwrap_or(&"").to_string()
                        } else {
                            slug_list.first().unwrap_or(&"").to_string()
                        };
                        rsx! {
                    div { class: "compare-column-header",
                        div { class: "compare-column-title-row",
                            select {
                                class: "compare-theme-picker",
                                value: "{current_slug}",
                                onchange: {
                                    let provider = provider.clone();
                                    move |evt: Event<FormData>| {
                                        let new_slug = evt.value();
                                        let new_slugs = if theme_idx == 0 {
                                            format!("{},{}", new_slug, other_slug)
                                        } else {
                                            format!("{},{}", other_slug, new_slug)
                                        };
                                        nav.replace(Route::CompareThemes {
                                            provider: provider.clone(),
                                            slugs: new_slugs,
                                        });
                                    }
                                },
                                for (slug, name) in &picker_opts {
                                    option {
                                        value: "{slug}",
                                        selected: *slug == current_slug,
                                        "{name}"
                                    }
                                }
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
                        if !cdata.rules.is_empty() {
                            div { class: "compare-chips",
                                for rule in &cdata.rules {
                                    {
                                        let rule_id = rule.id.clone();
                                        let rule_id_click = rule.id.clone();
                                        let fixtures_for_rule = cdata.fixtures_per_rule
                                            .get(&rule.id).cloned().unwrap_or_default();
                                        let fixture_count = fixtures_for_rule.len();
                                        let is_active = focused.as_ref()
                                            == Some(&(theme_idx, rule_id.clone()));
                                        let chip_class = if is_active {
                                            "detail-issue-chip detail-issue-chip-active"
                                        } else {
                                            "detail-issue-chip"
                                        };
                                        rsx! {
                                            button {
                                                class: "{chip_class}",
                                                onclick: move |_| {
                                                    let current = active_issue.read().clone();
                                                    match current {
                                                        Some((ti, ref id, idx)) if ti == theme_idx && *id == rule_id_click => {
                                                            if fixtures_for_rule.len() <= 1 {
                                                                // Single or no fixture: toggle off
                                                                active_issue.set(None);
                                                            } else {
                                                                let next = (idx + 1) % fixtures_for_rule.len();
                                                                let fixture_id = &fixtures_for_rule[next];
                                                                let anchor = format!("scene-{fixture_id}");
                                                                dioxus::document::eval(&format!(
                                                                    "document.getElementById('{anchor}')?.scrollIntoView({{behavior:'smooth',block:'start'}})"
                                                                ));
                                                                active_issue.set(Some((theme_idx, rule_id_click.clone(), next)));
                                                            }
                                                        }
                                                        _ => {
                                                            if let Some(fixture_id) = fixtures_for_rule.first() {
                                                                let anchor = format!("scene-{fixture_id}");
                                                                dioxus::document::eval(&format!(
                                                                    "document.getElementById('{anchor}')?.scrollIntoView({{behavior:'smooth',block:'start'}})"
                                                                ));
                                                            }
                                                            active_issue.set(Some((theme_idx, rule_id_click.clone(), 0)));
                                                        }
                                                    }
                                                },
                                                span { class: "detail-issue-chip-id", "{rule_id}" }
                                                span {
                                                    class: "color-chip",
                                                    style: "background: {rule.fg_hex};",
                                                }
                                                " on "
                                                span {
                                                    class: "color-chip",
                                                    style: "background: {rule.bg_hex};",
                                                }
                                                span { class: "detail-issue-chip-ratio", " {rule.ratio:.1}:1" }
                                                if fixture_count > 0 {
                                                    span { class: "issue-chip-count", "{fixture_count}" }
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

            for fixture in all_fixtures {
                {
                    rsx! {
                        div { class: "compare-scene-group",
                            id: "scene-{fixture.id}",

                            div { class: "compare-grid",
                                for (theme_idx, (theme, cdata)) in compare_themes.iter().zip(contrast_data.iter()).enumerate() {
                                    {
                                        let fixture_issue_count = cdata.issues_per_fixture
                                            .get(&fixture.id)
                                            .map(|v| unique_issue_count(v))
                                            .unwrap_or(0);
                                        rsx! {
                                            div { class: "compare-grid-item",
                                                h3 { class: "compare-scene-name",
                                                    "{fixture.name}"
                                                    if fixture_issue_count > 0 {
                                                        span { class: "scene-tab-badge", "{fixture_issue_count}" }
                                                    }
                                                }
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
                                                        let col_focused = focused.as_ref()
                                                            .and_then(|(ti, rid)| if *ti == theme_idx { Some(rid.clone()) } else { None });
                                                        rsx! {
                                                            term_renderer::TermOutputView {
                                                                theme: theme.clone(),
                                                                output: fixture.clone(),
                                                                issue_details: fixture_issues,
                                                                focused_rule: col_focused,
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
