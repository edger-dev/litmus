use dioxus::prelude::*;

use crate::components::*;
use crate::fixtures;
use crate::screenshot_view::{has_screenshot_for_provider, ScreenshotImage};
use crate::state::*;
use crate::term_renderer::{self, build_issue_registry, SpanIssueDetail};
use crate::themes;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page — all scenes rendered vertically.
#[component]
pub fn ThemeDetail(provider: String, slug: String) -> Element {
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
            let (rules, id_map) = build_issue_registry(&issues);
            let issue_count = rules.len();

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

            // Active issue for click-to-cycle: (rule_id, fixture_cycle_index)
            let mut active_issue: Signal<Option<(String, usize)>> = use_signal(|| None);

            // Group issues per fixture as (line, span, detail) tuples with rule IDs
            let mut issues_per_fixture: std::collections::HashMap<&str, Vec<(usize, usize, SpanIssueDetail)>> = std::collections::HashMap::new();
            for issue in &issues {
                let rule_id = id_map.get(&(issue.fg_term, issue.bg_term)).cloned();
                issues_per_fixture.entry(issue.fixture_id.as_str()).or_default().push(
                    (issue.line, issue.span, SpanIssueDetail {
                        rule_id,
                        ratio: issue.ratio,
                        threshold: issue.threshold,
                        fg_hex: issue.fg.to_hex(),
                        bg_hex: issue.bg.to_hex(),
                    })
                );
            }

            // Publish per-fixture unique issue counts to context for the minimap
            // Clear compare dots (we're on detail page, not compare)
            let mut compare_dots = use_context::<Signal<CompareIssueDots>>();
            if !compare_dots.read().0.is_empty() {
                compare_dots.set(CompareIssueDots::default());
            }
            let mut scene_issue_counts = use_context::<Signal<SceneIssueCounts>>();
            let counts: std::collections::HashMap<String, usize> = issues_per_fixture
                .iter()
                .map(|(k, v)| {
                    let unique: std::collections::HashSet<Option<&str>> = v
                        .iter()
                        .map(|(_, _, d)| d.rule_id.as_deref())
                        .collect();
                    (k.to_string(), unique.len())
                })
                .collect();
            if counts != scene_issue_counts.read().0 {
                scene_issue_counts.set(SceneIssueCounts(counts));
            }

            // Build per-rule list of fixture IDs for cycling
            let fixtures_per_rule: std::collections::HashMap<String, Vec<String>> = {
                let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
                for fixture in all_fixtures {
                    if let Some(fixture_issues) = issues_per_fixture.get(fixture.id.as_str()) {
                        let rule_ids: std::collections::HashSet<&str> = fixture_issues
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

            // Read active state for rendering
            let focused_rule_id: Option<String> = active_issue.read().as_ref().map(|(id, _)| id.clone());

            // Scroll to #fixture-id anchor on mount
            use_effect(move || {
                dioxus::document::eval(
                    r#"
                    const hash = window.location.hash.replace('#', '');
                    if (hash) {
                        const el = document.getElementById('scene-' + hash) || document.getElementById(hash);
                        if (el) el.scrollIntoView({ behavior: 'instant', block: 'start' });
                    }
                    "#
                );
            });

            // Update URL hash as user scrolls through fixtures
            let visible_scenes = use_context::<Signal<VisibleScenes>>();
            use_effect(move || {
                let visible = visible_scenes.read().0.clone();
                if visible.is_empty() { return; }
                // Pick the first visible fixture in document order
                let fixture_order: Vec<&str> = all_fixtures.iter().map(|f| f.id.as_str()).collect();
                let first_visible = fixture_order.iter().find(|id| visible.contains(**id));
                if let Some(id) = first_visible {
                    let js = format!(
                        "if (window.location.hash !== '#{id}') history.replaceState(null, '', window.location.pathname + '#{id}')"
                    );
                    dioxus::document::eval(&js);
                }
            });

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
                            Key::Escape => {
                                active_issue.set(None);
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
                                for rule in &rules {
                                    {
                                        let rule_id = rule.id.clone();
                                        let rule_id_click = rule.id.clone();
                                        let fixtures_for_rule = fixtures_per_rule.get(&rule.id).cloned().unwrap_or_default();
                                        let fixture_count = fixtures_for_rule.len();
                                        let is_active = focused_rule_id.as_ref() == Some(&rule.id);
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
                                                        Some((ref id, idx)) if *id == rule_id_click => {
                                                            if fixtures_for_rule.is_empty() {
                                                                active_issue.set(None);
                                                            } else {
                                                                let next = (idx + 1) % fixtures_for_rule.len();
                                                                // Scroll to next fixture
                                                                let fixture_id = &fixtures_for_rule[next];
                                                                let anchor = format!("scene-{fixture_id}");
                                                                dioxus::document::eval(&format!(
                                                                    "document.getElementById('{anchor}')?.scrollIntoView({{behavior:'smooth',block:'start'}})"
                                                                ));
                                                                active_issue.set(Some((rule_id_click.clone(), next)));
                                                            }
                                                        }
                                                        _ => {
                                                            // First click: activate and scroll to first fixture
                                                            if let Some(fixture_id) = fixtures_for_rule.first() {
                                                                let anchor = format!("scene-{fixture_id}");
                                                                dioxus::document::eval(&format!(
                                                                    "document.getElementById('{anchor}')?.scrollIntoView({{behavior:'smooth',block:'start'}})"
                                                                ));
                                                            }
                                                            active_issue.set(Some((rule_id_click.clone(), 0)));
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

                    // All fixtures rendered as side-by-side (terminal output + screenshot)
                    for fixture in all_fixtures {
                        {
                            let fixture_issues = issues_per_fixture
                                .get(fixture.id.as_str())
                                .cloned()
                                .unwrap_or_default();
                            let fixture_issue_count = {
                                let unique: std::collections::HashSet<Option<&str>> = fixture_issues
                                    .iter()
                                    .map(|(_, _, d)| d.rule_id.as_deref())
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
                                                focused_rule: focused_rule_id.clone(),
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
                    Link { to: crate::Route::ThemeList { provider: active_provider.read().0.clone() }, class: "accent-link", "Back to all themes" }
                }
            }
        }
    }
}
