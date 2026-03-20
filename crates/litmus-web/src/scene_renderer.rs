use dioxus::prelude::*;
use litmus_model::scene::{Scene, SceneLine, StyledSpan};
use litmus_model::Theme;

/// Render a complete scene as a terminal-style HTML block.
#[component]
pub fn SceneView(theme: Theme, scene: Scene) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let container_style = format!(
        "background-color: {bg}; color: {fg};"
    );

    rsx! {
        div { class: "scene-block",
            div {
                style: "margin-bottom: 0.5rem; font-weight: bold; \
                        font-size: 0.85rem; opacity: 0.7;",
                "{scene.name}"
            }
            pre {
                style: "{container_style}",
                for (i, line) in scene.lines.iter().enumerate() {
                    LineView { key: "{i}", theme: theme.clone(), line: line.clone() }
                    "\n"
                }
            }
        }
    }
}

/// Render a single scene line.
#[component]
fn LineView(theme: Theme, line: SceneLine) -> Element {
    if line.spans.is_empty() {
        return rsx! { "" };
    }

    rsx! {
        for (i, span) in line.spans.iter().enumerate() {
            SpanView { key: "{i}", theme: theme.clone(), span: span.clone() }
        }
    }
}

/// Render a single styled span as an HTML <span> with inline styles.
#[component]
fn SpanView(theme: Theme, span: StyledSpan) -> Element {
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

    rsx! {
        span { style: "{style_str}", "{span.text}" }
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
