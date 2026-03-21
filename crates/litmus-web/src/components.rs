use dioxus::prelude::*;
use dioxus::document::eval;
use litmus_model::cvd::CvdType;

use crate::state::*;

/// Circular score ring (donut gauge) rendered as inline SVG.
#[component]
pub fn ScoreRing(score: u8, size: f64) -> Element {
    let radius = 40.0_f64;
    let stroke_width = 10.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let progress = (score as f64 / 100.0).min(1.0);
    let dash_filled = circumference * progress;
    let dash_gap = circumference - dash_filled;
    let viewbox_size = (radius + stroke_width) * 2.0;
    let center = viewbox_size / 2.0;

    // Color: red < 70, orange < 85, green >= 85
    let ring_color = if score < 70 {
        "var(--app-error, #ff6b6b)"
    } else if score < 85 {
        "var(--app-warning, #ffa94d)"
    } else {
        "var(--app-success, #6bcb77)"
    };

    let dash_array = format!("{dash_filled} {dash_gap}");
    let dash_offset = format!("{}", circumference * 0.25); // start from top
    let size_px = format!("{size}px");
    let viewbox = format!("0 0 {viewbox_size} {viewbox_size}");

    rsx! {
        svg {
            class: "score-ring",
            width: "{size_px}",
            height: "{size_px}",
            view_box: "{viewbox}",
            // Background ring
            circle {
                cx: "{center}",
                cy: "{center}",
                r: "{radius}",
                fill: "none",
                stroke: "currentColor",
                stroke_opacity: "0.15",
                stroke_width: "{stroke_width}",
            }
            // Foreground ring
            circle {
                cx: "{center}",
                cy: "{center}",
                r: "{radius}",
                fill: "none",
                stroke: "{ring_color}",
                stroke_width: "{stroke_width}",
                stroke_dasharray: "{dash_array}",
                stroke_dashoffset: "{dash_offset}",
                stroke_linecap: "round",
                transform: "rotate(-90 {center} {center})",
            }
            // Percentage text
            text {
                x: "{center}",
                y: "{center}",
                text_anchor: "middle",
                dominant_baseline: "central",
                fill: "currentColor",
                font_size: "{radius * 0.7}",
                font_weight: "bold",
                font_family: "inherit",
                "{score}%"
            }
        }
    }
}


#[component]
pub fn FilterButton(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: if active { "filter-btn filter-btn-active" } else { "filter-btn" },
            aria_pressed: if active { "true" } else { "false" },
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

#[component]
pub fn ColorSwatch(label: String, color: String) -> Element {
    rsx! {
        div { class: "color-label",
            div {
                class: "color-chip",
                style: "background: {color};",
            }
            span { "{label}" }
        }
    }
}

/// Checkbox to add/remove a theme from the shortlist (used on cards).
#[component]
pub fn ShortlistCheckbox(slug: String, name: String) -> Element {
    let mut shortlist = use_context::<Signal<Shortlist>>();
    let app_theme = use_context::<Signal<AppThemeSlug>>();
    let is_current = app_theme.read().0.as_deref() == Some(&slug);
    let is_selected = shortlist.read().0.contains(&slug);

    let slug_for_click = slug.clone();
    rsx! {
        label {
            class: {
                let mut cls = String::from("shortlist-checkbox");
                if is_current { cls.push_str(" shortlist-checkbox-disabled"); }
                else if is_selected { cls.push_str(" shortlist-checkbox-active"); }
                cls
            },
            onclick: move |evt: Event<MouseData>| {
                evt.stop_propagation();
            },
            input {
                r#type: "checkbox",
                checked: is_selected || is_current,
                disabled: is_current,
                onchange: move |_| {
                    let mut sel = shortlist.write();
                    if let Some(pos) = sel.0.iter().position(|s| s == &slug_for_click) {
                        sel.0.remove(pos);
                    } else {
                        if sel.0.len() >= MAX_SHORTLIST {
                            sel.0.remove(0);
                        }
                        sel.0.push(slug_for_click.clone());
                    }
                },
            }
            span { if is_current { "Current" } else { "Shortlist" } }
        }
    }
}

/// Button to add/remove a theme from the shortlist (used on detail page).
#[component]
pub fn ShortlistToggle(slug: String, name: String) -> Element {
    let mut shortlist = use_context::<Signal<Shortlist>>();
    let app_theme = use_context::<Signal<AppThemeSlug>>();
    let is_current = app_theme.read().0.as_deref() == Some(&slug);
    let is_selected = shortlist.read().0.contains(&slug);

    let slug_for_click = slug.clone();
    rsx! {
        button {
            class: {
                let mut cls = String::from("shortlist-toggle");
                if is_current { cls.push_str(" shortlist-toggle-disabled"); }
                else if is_selected { cls.push_str(" shortlist-toggle-active"); }
                cls
            },
            aria_pressed: if is_selected || is_current { "true" } else { "false" },
            disabled: is_current,
            onclick: move |evt: Event<MouseData>| {
                evt.stop_propagation();
                let mut sel = shortlist.write();
                if let Some(pos) = sel.0.iter().position(|s| s == &slug_for_click) {
                    sel.0.remove(pos);
                } else if sel.0.len() < MAX_SHORTLIST {
                    sel.0.push(slug_for_click.clone());
                }
            },
            if is_current { "Current" } else if is_selected { "Shortlisted" } else { "+ Shortlist" }
        }
    }
}

/// Button to use a theme as the app chrome theme.
#[component]
pub fn UseAsAppThemeButton(slug: String) -> Element {
    let mut app_theme = use_context::<Signal<AppThemeSlug>>();
    let mut shortlist = use_context::<Signal<Shortlist>>();
    let is_active = app_theme.read().0.as_deref() == Some(&slug);

    let slug_for_click = slug.clone();
    rsx! {
        button {
            class: if is_active { "use-as-app-theme-btn use-as-app-theme-btn-active" } else { "use-as-app-theme-btn" },
            onclick: move |evt: Event<MouseData>| {
                evt.stop_propagation();
                if is_active {
                    app_theme.set(AppThemeSlug(None));
                } else {
                    // Push the previous app theme to the top of shortlist
                    if let Some(prev) = app_theme.read().0.clone() {
                        let mut sel = shortlist.write();
                        // Remove if already in shortlist (we'll re-insert at front)
                        sel.0.retain(|s| s != &prev);
                        sel.0.insert(0, prev);
                        // Trim to max
                        sel.0.truncate(MAX_SHORTLIST);
                    }
                    app_theme.set(AppThemeSlug(Some(slug_for_click.clone())));
                }
            },
            if is_active { "\u{2713} Applied" } else { "Apply" }
        }
    }
}

/// CVD simulation selector.
#[component]
pub fn CvdSelector(cvd_signal: Signal<Option<CvdType>>) -> Element {
    let current = *cvd_signal.read();

    rsx! {
        div {
            class: "cvd-selector",
            span {
                class: "cvd-label",
                title: "Simulate color vision deficiency to check theme accessibility",
                "CVD"
            }
            FilterButton {
                label: "Normal",
                active: current.is_none(),
                onclick: move |_| cvd_signal.set(None),
            }
            for cvd_type in CvdType::all() {
                {
                    let ct = *cvd_type;
                    let label = ct.label();
                    let desc = ct.description();
                    rsx! {
                        button {
                            class: if current == Some(ct) { "filter-btn filter-btn-active" } else { "filter-btn" },
                            aria_pressed: if current == Some(ct) { "true" } else { "false" },
                            title: "{desc}",
                            onclick: move |_| cvd_signal.set(Some(ct)),
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}

/// Export format selector with copy-to-clipboard buttons.
#[component]
pub fn ExportButtons(theme: litmus_model::Theme) -> Element {
    let mut active_format = use_signal(|| Option::<&'static str>::None);
    let mut copied = use_signal(|| false);

    let format = *active_format.read();
    let is_copied = *copied.read();

    let content = format.map(|f| match f {
        "kitty" => litmus_model::export::to_kitty_conf(&theme),
        "toml" => litmus_model::export::to_toml(&theme),
        "nix" => litmus_model::export::to_nix(&theme),
        _ => String::new(),
    });

    rsx! {
        div { class: "export-section",
            div { class: "export-header",
                h3 { class: "export-title", "Export" }

                ExportFormatBtn {
                    label: "kitty.conf",
                    active: format == Some("kitty"),
                    onclick: move |_| {
                        active_format.set(Some("kitty"));
                        copied.set(false);
                    },
                }
                ExportFormatBtn {
                    label: "TOML",
                    active: format == Some("toml"),
                    onclick: move |_| {
                        active_format.set(Some("toml"));
                        copied.set(false);
                    },
                }
                ExportFormatBtn {
                    label: "Nix",
                    active: format == Some("nix"),
                    onclick: move |_| {
                        active_format.set(Some("nix"));
                        copied.set(false);
                    },
                }

                button {
                    class: "export-btn",
                    onclick: move |_| {
                        let js = "navigator.clipboard.writeText(window.location.href)";
                        eval(js);
                        copied.set(true);
                    },
                    "Copy Link"
                }

                if is_copied {
                    span { class: "copied-indicator", "Copied!" }
                }
            }

            if let Some(text) = &content {
                div { class: "export-content",
                    button {
                        class: "export-copy-btn",
                        onclick: {
                            let text = text.clone();
                            move |_| {
                                let escaped = text.replace('\\', "\\\\")
                                    .replace('`', "\\`")
                                    .replace('$', "\\$");
                                let js = format!("navigator.clipboard.writeText(`{escaped}`)");
                                eval(&js);
                                copied.set(true);
                            }
                        },
                        if is_copied { "Copied!" } else { "Copy" }
                    }

                    pre { class: "mono export-pre", "{text}" }
                }
            }
        }
    }
}

#[component]
fn ExportFormatBtn(
    label: &'static str,
    active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: if active { "export-btn export-btn-active" } else { "export-btn" },
            aria_pressed: if active { "true" } else { "false" },
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}

/// Scene minimap — fixed vertical strip on the right edge showing all scene names.
/// Highlights scenes currently visible in the viewport via IntersectionObserver.
#[component]
pub fn SceneMinimap(scenes: Vec<litmus_model::scene::Scene>, #[props(default = true)] show_badges: bool) -> Element {
    let mut visible = use_context::<Signal<VisibleScenes>>();
    let scene_issue_counts = use_context::<Signal<SceneIssueCounts>>();

    // Set up IntersectionObserver on mount to track which scenes are in view
    use_effect(move || {
        let scene_ids: Vec<String> = litmus_model::scenes::all_scenes()
            .iter()
            .map(|s| s.id.clone())
            .collect();
        // Build JS array literal from scene IDs
        let ids_js: Vec<String> = scene_ids.iter().map(|id| format!("\"{}\"", id)).collect();
        let ids_array = format!("[{}]", ids_js.join(","));
        let js = format!(
            r#"
            window.__litmus_visible_scenes = {{}};
            const ids = {ids_array};
            if (window.__litmus_minimap_observer) {{
                window.__litmus_minimap_observer.disconnect();
            }}
            const observer = new IntersectionObserver((entries) => {{
                entries.forEach(e => {{
                    const id = e.target.id.replace('scene-', '');
                    if (e.isIntersecting) {{
                        window.__litmus_visible_scenes[id] = true;
                    }} else {{
                        delete window.__litmus_visible_scenes[id];
                    }}
                }});
            }}, {{ threshold: 0.1 }});
            ids.forEach(id => {{
                const el = document.getElementById('scene-' + id);
                if (el) observer.observe(el);
            }});
            window.__litmus_minimap_observer = observer;
            "#
        );
        eval(&js);
    });

    // Poll visible scenes from JS every 200ms using eval-based sleep
    use_future(move || async move {
        loop {
            let js = r#"
                await new Promise(r => setTimeout(r, 200));
                return Object.keys(window.__litmus_visible_scenes || {}).join(",");
            "#;
            if let Ok(result) = eval(js).await {
                let csv = result.to_string();
                // Result is a JSON string like "\"id1,id2\"" — strip quotes
                let csv = csv.trim_matches('"');
                let set: std::collections::HashSet<String> = if csv.is_empty() {
                    std::collections::HashSet::new()
                } else {
                    csv.split(',').map(|s| s.to_string()).collect()
                };
                if set != visible.read().0 {
                    visible.set(VisibleScenes(set));
                }
            }
        }
    });

    let visible_set = visible.read().0.clone();
    let issue_counts = scene_issue_counts.read().0.clone();

    rsx! {
        nav { class: "scene-minimap",
            aria_label: "Scene navigation",
            for scene in &scenes {
                {
                    let is_visible = visible_set.contains(&scene.id);
                    let scene_id = scene.id.clone();
                    let issue_count = issue_counts.get(&scene.id).copied().unwrap_or(0);
                    rsx! {
                        button {
                            class: if is_visible { "scene-minimap-item scene-minimap-item-active" } else { "scene-minimap-item" },
                            onclick: move |_| {
                                let js = format!(
                                    "document.getElementById('scene-{}').scrollIntoView({{behavior:'smooth',block:'start'}})",
                                    scene_id
                                );
                                eval(&js);
                            },
                            "{scene.name}"
                            if show_badges && issue_count > 0 {
                                span { class: "scene-tab-badge", "{issue_count}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

