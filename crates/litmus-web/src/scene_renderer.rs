use dioxus::prelude::*;
use litmus_model::scene::{Scene, SceneLine, StyledSpan};
use litmus_model::Theme;

/// Render a complete scene as a terminal-style HTML block.
///
/// `issue_spans` contains `(line_idx, span_idx)` pairs marking spans with
/// contrast issues. Those spans get a visual indicator via CSS.
#[component]
pub fn SceneView(
    theme: Theme,
    scene: Scene,
    #[props(default = false)] compact: bool,
    #[props(default)] issue_spans: Vec<(usize, usize)>,
) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let class = if compact { "scene-block scene-compact" } else { "scene-block" };

    rsx! {
        div { class: "{class}",
            if !compact {
                div {
                    style: "margin-bottom: 0.5rem; font-weight: bold; \
                            font-size: 0.85rem; opacity: 0.7;",
                    "{scene.name}"
                }
            }
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in scene.lines.iter().enumerate() {
                    LineView { key: "{i}", theme: theme.clone(), line: line.clone(), line_idx: i, issue_spans: issue_spans.clone() }
                    "\n"
                }
            }
        }
    }
}

/// Render a scene preview: first N lines only, no title, compact styling.
#[component]
pub fn ScenePreview(theme: Theme, scene: Scene, #[props(default = 5)] max_lines: usize) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let lines_to_show = scene.lines.len().min(max_lines);

    rsx! {
        div { class: "scene-preview",
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in scene.lines.iter().take(lines_to_show).enumerate() {
                    LineView { key: "{i}", theme: theme.clone(), line: line.clone() }
                    "\n"
                }
            }
        }
    }
}

/// Render a single scene line.
#[component]
fn LineView(theme: Theme, line: SceneLine, #[props(default)] line_idx: usize, #[props(default)] issue_spans: Vec<(usize, usize)>) -> Element {
    if line.spans.is_empty() {
        return rsx! { "" };
    }

    rsx! {
        for (i, span) in line.spans.iter().enumerate() {
            {
                let has_issue = issue_spans.iter().any(|(l, s)| *l == line_idx && *s == i);
                rsx! {
                    SpanView { key: "{i}", theme: theme.clone(), span: span.clone(), has_issue: has_issue }
                }
            }
        }
    }
}

/// Render a single styled span as an HTML <span> with inline styles.
#[component]
fn SpanView(theme: Theme, span: StyledSpan, #[props(default = false)] has_issue: bool) -> Element {
    let mut styles = Vec::new();

    if let Some(ref fg) = span.fg {
        let color = fg.resolve(&theme).to_hex();
        styles.push(format!("color: {color}"));
    }

    if let Some(ref bg) = span.bg {
        let color = bg.resolve(&theme).to_hex();
        styles.push(format!("background-color: {color}"));
    }

    if span.style.bold {
        styles.push("font-weight: bold".into());
    }
    if span.style.italic {
        styles.push("font-style: italic".into());
    }
    if span.style.underline {
        styles.push("text-decoration: underline".into());
    }
    if span.style.dim {
        styles.push("opacity: 0.6".into());
    }

    let style_str = styles.join("; ");
    let class = if has_issue { "contrast-issue-span" } else { "" };

    rsx! {
        span { class: "{class}", style: "{style_str}", "{span.text}" }
    }
}

/// Render all scenes for a given theme.
#[component]
pub fn AllScenesView(theme: Theme) -> Element {
    let scenes = litmus_model::scenes::all_scenes();

    rsx! {
        div { class: "scenes-container",
            for scene in scenes {
                SceneView { key: "{scene.id}", theme: theme.clone(), scene: scene }
            }
        }
    }
}
