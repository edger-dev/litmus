use dioxus::prelude::*;
use litmus_model::term_output::{TermColor, TermLine, TermOutput, TermSpan};
use litmus_model::Theme;

/// Render a complete TermOutput as a terminal-style HTML block.
#[component]
pub fn TermOutputView(
    theme: Theme,
    output: TermOutput,
    #[props(default = false)] compact: bool,
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
                    TermLineView { key: "{i}", theme: theme.clone(), line: line.clone() }
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
fn TermLineView(theme: Theme, line: TermLine) -> Element {
    if line.spans.is_empty() {
        return rsx! { "" };
    }

    rsx! {
        for (i, span) in line.spans.iter().enumerate() {
            TermSpanView { key: "{i}", theme: theme.clone(), span: span.clone() }
        }
    }
}

/// Render a single TermSpan as an HTML <span> with inline styles.
#[component]
fn TermSpanView(theme: Theme, span: TermSpan) -> Element {
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

    rsx! {
        span { style: "{style_str}", "{span.text}" }
    }
}
