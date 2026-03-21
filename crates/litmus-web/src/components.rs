use dioxus::prelude::*;
use dioxus::document::eval;
use litmus_model::cvd::CvdType;

use crate::state::*;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

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
                    } else if sel.0.len() < MAX_SHORTLIST {
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

/// Color diff table showing which colors differ between compared themes.
#[component]
pub fn ColorDiffTable(themes: Vec<litmus_model::Theme>) -> Element {
    let mut expanded = use_signal(|| false);
    let is_expanded = *expanded.read();

    let mut diff_rows: Vec<(String, Vec<String>, bool)> = Vec::new();

    let add_row = |rows: &mut Vec<(String, Vec<String>, bool)>,
                   name: &str,
                   values: Vec<String>| {
        let differs = values.windows(2).any(|w| w[0] != w[1]);
        rows.push((name.to_string(), values, differs));
    };

    add_row(&mut diff_rows, "bg", themes.iter().map(|t| t.background.to_hex()).collect());
    add_row(&mut diff_rows, "fg", themes.iter().map(|t| t.foreground.to_hex()).collect());
    add_row(&mut diff_rows, "cursor", themes.iter().map(|t| t.cursor.to_hex()).collect());

    for (i, name) in ANSI_NAMES.iter().enumerate() {
        let values: Vec<String> = themes
            .iter()
            .map(|t| t.ansi.as_array()[i].to_hex())
            .collect();
        add_row(&mut diff_rows, name, values);
    }

    let diff_count = diff_rows.iter().filter(|(_, _, d)| *d).count();

    rsx! {
        div { class: "color-diff-table",
            div {
                class: "color-diff-header",
                onclick: move |_| expanded.set(!is_expanded),

                span { class: "mono", "Color differences: {diff_count}/19" }
                span { class: "mono color-diff-toggle",
                    if is_expanded { "collapse" } else { "expand" }
                }
            }

            if is_expanded {
                div { class: "color-diff-body",
                    table { class: "mono color-diff-grid",
                        thead {
                            tr {
                                th { class: "color-diff-cell", "Color" }
                                for theme in &themes {
                                    th { class: "color-diff-cell", "{theme.name}" }
                                }
                            }
                        }

                        tbody {
                            for (name, values, differs) in &diff_rows {
                                tr {
                                    class: if *differs { "color-diff-row-changed" } else { "" },
                                    td { class: "color-diff-cell color-diff-name", "{name}" }
                                    for val in values {
                                        td { class: "color-diff-cell",
                                            div { class: "color-diff-value",
                                                div {
                                                    class: "color-diff-chip",
                                                    style: "background: {val};",
                                                }
                                                span { class: "color-diff-hex", "{val}" }
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
