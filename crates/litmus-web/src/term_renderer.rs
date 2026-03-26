use dioxus::prelude::*;
use litmus_model::term_output::{TermColor, TermLine, TermOutput, TermSpan};
use litmus_model::Theme;

/// Detail about a contrast issue on a specific span, used for tooltips.
#[derive(Clone, PartialEq)]
pub struct SpanIssueDetail {
    /// WCAG 2.x contrast ratio (informational).
    pub ratio: f64,
    /// APCA |Lc| threshold that was not met.
    pub threshold: f64,
    /// Resolved foreground hex color.
    pub fg_hex: String,
    /// Resolved background hex color.
    pub bg_hex: String,
}

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
) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let class = if compact {
        "scene-block scene-compact"
    } else {
        "scene-block"
    };

    rsx! {
        div { class: "{class}",
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in output.lines.iter().enumerate() {
                    TermLineView {
                        key: "{i}",
                        theme: theme.clone(),
                        line: line.clone(),
                        line_idx: i,
                        issue_details: issue_details.clone(),
                    }
                    "\n"
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
                    }
                }
            }
        }
    }
}

/// Render a single TermSpan as an HTML <span> with inline styles.
#[component]
fn TermSpanView(
    theme: Theme,
    span: TermSpan,
    #[props(default)] line_idx: usize,
    #[props(default)] issue_detail: Option<SpanIssueDetail>,
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
    let has_issue = issue_detail.is_some();
    let class = if has_issue { "contrast-issue-span" } else { "" };

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
                    rsx! {
                        span { class: "{tooltip_class}",
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
