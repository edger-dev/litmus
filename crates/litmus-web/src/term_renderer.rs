use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use litmus_model::term_output::{TermColor, TermLine, TermOutput, TermSpan};
use litmus_model::Theme;

// ── ANSI color names ────────────────────────────────────────────────

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Human-readable label for a TermColor variant.
pub fn term_color_label(tc: &TermColor) -> String {
    match tc {
        TermColor::Default => "default".into(),
        TermColor::Ansi(i) => ANSI_NAMES.get(*i as usize).unwrap_or(&"ansi").to_string(),
        TermColor::Indexed(i) => format!("idx-{i}"),
        TermColor::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
    }
}

// ── Issue registry ──────────────────────────────────────────────────

/// A unique contrast rule violation identified by TermColor pair.
#[derive(Clone, PartialEq)]
pub struct ContrastRule {
    pub id: String,
    pub fg_term: TermColor,
    pub bg_term: TermColor,
    pub fg_hex: String,
    pub bg_hex: String,
    pub ratio: f64,
    pub label: String,
}

/// Build a registry of unique contrast rules from all issues.
/// Returns (rules sorted by ID, map from (fg_term, bg_term) → rule_id).
pub fn build_issue_registry(
    issues: &[litmus_model::contrast::TermContrastIssue],
) -> (Vec<ContrastRule>, HashMap<(TermColor, TermColor), String>) {
    let mut seen: HashMap<(TermColor, TermColor), usize> = HashMap::new();
    let mut rules: Vec<ContrastRule> = Vec::new();

    for issue in issues {
        let key = (issue.fg_term, issue.bg_term);
        if let std::collections::hash_map::Entry::Vacant(entry) = seen.entry(key) {
            let idx = rules.len() + 1;
            let id = format!("C{idx}");
            let fg_label = term_color_label(&issue.fg_term);
            let bg_label = term_color_label(&issue.bg_term);
            let label = format!("{fg_label} on {bg_label}");
            entry.insert(rules.len());
            rules.push(ContrastRule {
                id: id.clone(),
                fg_term: issue.fg_term,
                bg_term: issue.bg_term,
                fg_hex: issue.fg.to_hex(),
                bg_hex: issue.bg.to_hex(),
                ratio: issue.ratio,
                label,
            });
        }
    }

    let id_map: HashMap<(TermColor, TermColor), String> = seen
        .into_iter()
        .map(|(key, idx)| (key, rules[idx].id.clone()))
        .collect();

    (rules, id_map)
}

// ── Footnote merging ────────────────────────────────────────────────

/// A merged footnote position: show tag at the end of `last_line` for a
/// contiguous block of the same rule at the same span index.
#[derive(Clone, PartialEq, Debug)]
pub struct MergedFootnote {
    pub line: usize,
    pub rule_id: String,
}

/// Compute merged footnote positions for a fixture.
///
/// Groups runs of consecutive lines where the same rule appears at the same
/// span index, then emits one footnote at the last line of each run.
/// Multiple rules on the same line are combined.
pub fn compute_footnotes(
    issue_details: &[(usize, usize, SpanIssueDetail)],
) -> Vec<MergedFootnote> {
    // Group by (rule_id, span_idx) → sorted list of line indices
    let mut runs: HashMap<(String, usize), Vec<usize>> = HashMap::new();
    for (line, span, detail) in issue_details {
        if let Some(ref rule_id) = detail.rule_id {
            runs.entry((rule_id.clone(), *span))
                .or_default()
                .push(*line);
        }
    }

    // For each group, find contiguous runs and emit footnote at last line of each
    let mut footnotes_per_line: HashMap<usize, HashSet<String>> = HashMap::new();
    for ((rule_id, _), mut lines) in runs {
        lines.sort();
        lines.dedup();
        // Split into contiguous runs
        let mut i = 0;
        while i < lines.len() {
            let mut j = i;
            while j + 1 < lines.len() && lines[j + 1] == lines[j] + 1 {
                j += 1;
            }
            // Footnote at the last line of this contiguous run
            footnotes_per_line
                .entry(lines[j])
                .or_default()
                .insert(rule_id.clone());
            i = j + 1;
        }
    }

    let mut result: Vec<MergedFootnote> = Vec::new();
    let mut sorted_lines: Vec<usize> = footnotes_per_line.keys().copied().collect();
    sorted_lines.sort();
    for line in sorted_lines {
        let mut ids: Vec<String> = footnotes_per_line[&line].iter().cloned().collect();
        ids.sort();
        for rule_id in ids {
            result.push(MergedFootnote { line, rule_id });
        }
    }
    result
}

// ── Span issue detail ───────────────────────────────────────────────

/// Detail about a contrast issue on a specific span, used for tooltips.
#[derive(Clone, PartialEq)]
pub struct SpanIssueDetail {
    /// Rule ID (e.g. "C1").
    pub rule_id: Option<String>,
    /// WCAG 2.x contrast ratio (informational).
    pub ratio: f64,
    /// APCA |Lc| threshold that was not met.
    pub threshold: f64,
    /// Resolved foreground hex color.
    pub fg_hex: String,
    /// Resolved background hex color.
    pub bg_hex: String,
}

// ── Components ──────────────────────────────────────────────────────

/// Render a complete TermOutput as a terminal-style HTML block.
///
/// `issue_details` contains `(line_idx, span_idx, detail)` tuples marking spans
/// with contrast issues. Those spans get a visual indicator + tooltip via CSS.
#[component]
pub fn TermOutputView(
    theme: Theme,
    output: TermOutput,
    #[props(default = false)] compact: bool,
    #[props(default)] issue_details: Vec<(usize, usize, SpanIssueDetail)>,
    #[props(default)] focused_rule: Option<String>,
) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let class = if compact {
        "scene-block scene-compact"
    } else {
        "scene-block"
    };

    let footnotes = compute_footnotes(&issue_details);

    rsx! {
        div { class: "{class}",
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in output.lines.iter().enumerate() {
                    {
                        let line_footnotes: Vec<&MergedFootnote> = footnotes
                            .iter()
                            .filter(|f| f.line == i)
                            .collect();
                        rsx! {
                            TermLineView {
                                key: "{i}",
                                theme: theme.clone(),
                                line: line.clone(),
                                line_idx: i,
                                issue_details: issue_details.clone(),
                                footnotes: line_footnotes.iter().map(|f| f.rule_id.clone()).collect::<Vec<_>>(),
                                focused_rule: focused_rule.clone(),
                            }
                            "\n"
                        }
                    }
                }
            }
        }
    }
}

/// Render a TermOutput preview: first N lines only, compact styling.
#[component]
pub fn TermOutputPreview(
    theme: Theme,
    output: TermOutput,
    #[props(default = 5)] max_lines: usize,
) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let lines_to_show = output.lines.len().min(max_lines);

    rsx! {
        div { class: "scene-preview",
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in output.lines.iter().take(lines_to_show).enumerate() {
                    TermLineView { key: "{i}", theme: theme.clone(), line: line.clone() }
                    "\n"
                }
            }
        }
    }
}

/// Render a single TermLine.
#[component]
fn TermLineView(
    theme: Theme,
    line: TermLine,
    #[props(default)] line_idx: usize,
    #[props(default)] issue_details: Vec<(usize, usize, SpanIssueDetail)>,
    #[props(default)] footnotes: Vec<String>,
    #[props(default)] focused_rule: Option<String>,
) -> Element {
    if line.spans.is_empty() {
        return rsx! { "" };
    }

    rsx! {
        for (i, span) in line.spans.iter().enumerate() {
            {
                let detail = issue_details.iter()
                    .find(|(l, s, _)| *l == line_idx && *s == i)
                    .map(|(_, _, d)| d.clone());
                rsx! {
                    TermSpanView {
                        key: "{i}",
                        theme: theme.clone(),
                        span: span.clone(),
                        line_idx: line_idx,
                        issue_detail: detail,
                        focused_rule: focused_rule.clone(),
                    }
                }
            }
        }
        // Footnote tags at end of line
        if !footnotes.is_empty() {
            span { class: "contrast-footnotes",
                for id in &footnotes {
                    {
                        let is_focused = focused_rule.as_ref() == Some(id);
                        let cls = if is_focused {
                            "contrast-footnote contrast-footnote-focused"
                        } else {
                            "contrast-footnote"
                        };
                        rsx! {
                            span { class: "{cls}", "{id}" }
                        }
                    }
                }
            }
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use litmus_model::contrast::TermContrastIssue;
    use litmus_model::Color;

    fn make_issue(fixture_id: &str, line: usize, span: usize, fg_term: TermColor, bg_term: TermColor) -> TermContrastIssue {
        TermContrastIssue {
            fixture_id: fixture_id.into(),
            line,
            span,
            text: "x".into(),
            fg: Color::new(255, 0, 0),
            bg: Color::new(0, 0, 0),
            fg_term,
            bg_term,
            ratio: 2.5,
            threshold: 30.0,
        }
    }

    #[test]
    fn build_issue_registry_empty() {
        let (rules, id_map) = build_issue_registry(&[]);
        assert!(rules.is_empty());
        assert!(id_map.is_empty());
    }

    #[test]
    fn build_issue_registry_deduplicates() {
        let issues = vec![
            make_issue("f1", 0, 0, TermColor::Ansi(1), TermColor::Default),
            make_issue("f1", 1, 0, TermColor::Ansi(1), TermColor::Default), // same pair
            make_issue("f2", 0, 0, TermColor::Ansi(2), TermColor::Default), // different pair
        ];
        let (rules, id_map) = build_issue_registry(&issues);
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].id, "C1");
        assert_eq!(rules[1].id, "C2");
        assert_eq!(id_map[&(TermColor::Ansi(1), TermColor::Default)], "C1");
        assert_eq!(id_map[&(TermColor::Ansi(2), TermColor::Default)], "C2");
    }

    #[test]
    fn compute_footnotes_empty() {
        assert!(compute_footnotes(&[]).is_empty());
    }

    #[test]
    fn compute_footnotes_merges_contiguous_lines() {
        // Same rule at same span on lines 0,1,2 → one footnote at line 2
        let details: Vec<(usize, usize, SpanIssueDetail)> = (0..3)
            .map(|line| (line, 0, SpanIssueDetail {
                rule_id: Some("C1".into()),
                ratio: 2.5,
                threshold: 30.0,
                fg_hex: "#ff0000".into(),
                bg_hex: "#000000".into(),
            }))
            .collect();
        let footnotes = compute_footnotes(&details);
        assert_eq!(footnotes.len(), 1);
        assert_eq!(footnotes[0].line, 2);
        assert_eq!(footnotes[0].rule_id, "C1");
    }

    #[test]
    fn compute_footnotes_splits_on_gap() {
        // Lines 0,1 then gap then 5,6 → two footnotes at lines 1 and 6
        let lines = [0, 1, 5, 6];
        let details: Vec<(usize, usize, SpanIssueDetail)> = lines
            .iter()
            .map(|&line| (line, 0, SpanIssueDetail {
                rule_id: Some("C1".into()),
                ratio: 2.5,
                threshold: 30.0,
                fg_hex: "#ff0000".into(),
                bg_hex: "#000000".into(),
            }))
            .collect();
        let footnotes = compute_footnotes(&details);
        assert_eq!(footnotes.len(), 2);
        assert_eq!(footnotes[0].line, 1);
        assert_eq!(footnotes[1].line, 6);
    }

    #[test]
    fn compute_footnotes_multiple_rules_same_line() {
        // Two rules both ending at line 0 → two footnotes at line 0, sorted by ID
        let details = vec![
            (0, 0, SpanIssueDetail {
                rule_id: Some("C2".into()),
                ratio: 2.5, threshold: 30.0,
                fg_hex: "#ff0000".into(), bg_hex: "#000000".into(),
            }),
            (0, 1, SpanIssueDetail {
                rule_id: Some("C1".into()),
                ratio: 2.5, threshold: 30.0,
                fg_hex: "#00ff00".into(), bg_hex: "#000000".into(),
            }),
        ];
        let footnotes = compute_footnotes(&details);
        assert_eq!(footnotes.len(), 2);
        assert_eq!(footnotes[0].rule_id, "C1");
        assert_eq!(footnotes[1].rule_id, "C2");
    }

    #[test]
    fn compute_footnotes_skips_none_rule_id() {
        let details = vec![
            (0, 0, SpanIssueDetail {
                rule_id: None,
                ratio: 2.5, threshold: 30.0,
                fg_hex: "#ff0000".into(), bg_hex: "#000000".into(),
            }),
        ];
        assert!(compute_footnotes(&details).is_empty());
    }
}

/// Render a single TermSpan as an HTML <span> with inline styles.
#[component]
fn TermSpanView(
    theme: Theme,
    span: TermSpan,
    #[props(default)] line_idx: usize,
    #[props(default)] issue_detail: Option<SpanIssueDetail>,
    #[props(default)] focused_rule: Option<String>,
) -> Element {
    let mut styles = Vec::new();

    // Resolve foreground color (skip for Default — inherits from parent <pre>)
    if span.fg != TermColor::Default {
        let color = span.fg.resolve_with_theme(&theme, &theme.foreground);
        styles.push(format!("color: {}", color.to_hex()));
    }

    // Resolve background color (skip for Default — inherits from parent <pre>)
    if span.bg != TermColor::Default {
        let color = span.bg.resolve_with_theme(&theme, &theme.background);
        styles.push(format!("background-color: {}", color.to_hex()));
    }

    if span.bold {
        styles.push("font-weight: bold".into());
    }
    if span.italic {
        styles.push("font-style: italic".into());
    }
    if span.underline {
        styles.push("text-decoration: underline".into());
    }
    if span.dim {
        styles.push("opacity: 0.6".into());
    }

    let style_str = styles.join("; ");

    let (has_issue, is_focused, is_dimmed) = match &issue_detail {
        Some(d) => {
            let rule_id = d.rule_id.as_deref();
            let focused = focused_rule.as_deref();
            match (rule_id, focused) {
                (Some(rid), Some(fid)) if rid == fid => (true, true, false),
                (Some(_), Some(_)) => (true, false, true),
                _ => (true, false, false),
            }
        }
        None => (false, false, false),
    };

    let class = match (has_issue, is_focused, is_dimmed) {
        (true, true, _) => "contrast-issue-span contrast-issue-focused",
        (true, _, true) => "contrast-issue-span contrast-issue-dimmed",
        (true, _, _) => "contrast-issue-span",
        _ => "",
    };

    rsx! {
        span { class: "{class}", style: "{style_str}",
            "{span.text}"
            if let Some(d) = &issue_detail {
                {
                    // Place tooltip below for first two lines (would overflow above)
                    let tooltip_class = if line_idx < 2 {
                        "contrast-tooltip contrast-tooltip-below"
                    } else {
                        "contrast-tooltip"
                    };
                    let rule_label = d.rule_id.as_deref().unwrap_or("");
                    rsx! {
                        span { class: "{tooltip_class}",
                            if !rule_label.is_empty() {
                                span { class: "contrast-tooltip-id", "{rule_label} " }
                            }
                            span { class: "contrast-tooltip-rule",
                                "APCA: requires |Lc| ≥ {d.threshold:.0} for readability"
                            }
                            br {}
                            span { class: "contrast-tooltip-ratio",
                                "WCAG ratio: {d.ratio:.1}:1"
                            }
                            br {}
                            span { class: "contrast-tooltip-colors",
                                span {
                                    class: "color-chip",
                                    style: "background: {d.fg_hex};",
                                }
                                " {d.fg_hex} on "
                                span {
                                    class: "color-chip",
                                    style: "background: {d.bg_hex};",
                                }
                                " {d.bg_hex}"
                            }
                        }
                    }
                }
            }
        }
    }
}
