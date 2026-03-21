use dioxus::prelude::*;

use crate::components::FilterButton;
use crate::state::*;
use crate::themes;
use crate::Route;

fn random_index(max: usize) -> usize {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Math::random() * max as f64).floor() as usize
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = max;
        0
    }
}


/// Persistent left sidebar: navigation, shortlist management, CVD toggle.
#[component]
pub fn Sidebar() -> Element {
    let all_themes = themes::load_embedded_themes();
    let mut shortlist = use_context::<Signal<Shortlist>>();
    let mut cvd_sim = use_context::<Signal<CvdSimulation>>();
    let mut sidebar_open = use_context::<Signal<SidebarOpen>>();
    let app_theme = use_context::<Signal<AppThemeSlug>>();
    let nav = navigator();

    let cvd = cvd_sim.read().0;
    let sl = shortlist.read().clone();
    let sl_count = sl.0.len();
    let app_slug = app_theme.read().0.clone();

    let has_shortlist = sl_count > 0;

    // Build compare URL: app theme + shortlist slugs (deduped)
    let mut compare_slugs: Vec<String> = Vec::new();
    if let Some(ref slug) = app_slug {
        compare_slugs.push(slug.clone());
    }
    for s in &sl.0 {
        if !compare_slugs.contains(s) {
            compare_slugs.push(s.clone());
        }
    }

    // If we have some themes but fewer than 2, fill to reach 2
    if has_shortlist && compare_slugs.len() < 2 {
        let mut fillers: Vec<String> = all_themes.iter()
            .map(|t| theme_slug(&t.name))
            .filter(|s| !compare_slugs.contains(s))
            .collect();
        fillers.sort();
        for f in fillers {
            if compare_slugs.len() >= 2 {
                break;
            }
            compare_slugs.push(f);
        }
    }

    let compare_label = if has_shortlist {
        format!("Compare ({})", compare_slugs.len())
    } else {
        "Feel Lucky".to_string()
    };
    let compare_url = compare_slugs.join(",");

    let show_shortlist = sl_count > 0 || app_slug.is_some();

    // Collect all theme slugs for the "Feel Lucky" random pick
    let all_slugs: Vec<String> = all_themes.iter().map(|t| theme_slug(&t.name)).collect();

    rsx! {
        aside {
            class: "sidebar",
            role: "navigation",
            aria_label: "Main navigation",

            // Header
            div { class: "sidebar-header",
                Link { to: Route::ThemeList {}, class: "sidebar-logo",
                    span { class: "sidebar-logo-text", "litmus" }
                }
                span { class: "sidebar-subtitle", "terminal color theme previewer" }
            }

            // Nav links
            div { class: "sidebar-section sidebar-nav",
                Link {
                    to: Route::ThemeList {},
                    class: "sidebar-nav-link",
                    onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                    "Browse Themes"
                }
                if has_shortlist {
                    Link {
                        to: Route::CompareThemes { slugs: compare_url.clone() },
                        class: "sidebar-nav-link",
                        onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                        "{compare_label}"
                    }
                } else {
                    button {
                        class: "sidebar-nav-link sidebar-feel-lucky",
                        onclick: move |_| {
                            sidebar_open.set(SidebarOpen(false));
                            if all_slugs.len() >= 2 {
                                // Pick a random theme different from current app theme
                                let current = app_theme.read().0.clone();
                                let candidates: Vec<&String> = all_slugs.iter()
                                    .filter(|s| current.as_deref() != Some(s.as_str()))
                                    .collect();
                                if candidates.is_empty() { return; }
                                let idx = random_index(candidates.len());
                                let random_slug = candidates[idx].clone();

                                // Add to shortlist
                                {
                                    let mut sel = shortlist.write();
                                    if !sel.0.contains(&random_slug) {
                                        sel.0.insert(0, random_slug.clone());
                                        sel.0.truncate(MAX_SHORTLIST);
                                    }
                                }

                                // Compare: current app theme + random (or two randoms if no app theme)
                                let compare = if let Some(ref cur) = current {
                                    format!("{},{}", cur, random_slug)
                                } else {
                                    let idx2 = (idx + 1 + random_index(candidates.len().saturating_sub(1).max(1))) % candidates.len();
                                    let random_slug2 = candidates[idx2].clone();
                                    format!("{},{}", random_slug, random_slug2)
                                };
                                nav.push(Route::CompareThemes { slugs: compare });
                            }
                        },
                        "{compare_label}"
                    }
                }
            }

            // Shortlist
            if show_shortlist {
                div { class: "sidebar-section sidebar-shortlist",
                    div { class: "sidebar-section-label", "Shortlist" }
                    div { class: "sidebar-shortlist-items",
                        // Pinned app theme entry
                        if let Some(ref app_s) = app_slug {
                            {
                                let name = all_themes.iter()
                                    .find(|t| theme_slug(&t.name) == *app_s)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| app_s.clone());
                                rsx! {
                                    div { class: "sidebar-shortlist-item sidebar-shortlist-current",
                                        Link {
                                            to: Route::ThemeDetail { slug: app_s.clone() },
                                            class: "sidebar-shortlist-name-link",
                                            onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                            span { class: "sidebar-shortlist-name", "{name}" }
                                        }
                                        span { class: "sidebar-current-badge", "current" }
                                    }
                                }
                            }
                        }

                        for slug in sl.0.iter().filter(|s| Some(s.as_str()) != app_slug.as_deref()) {
                            {
                                let name = all_themes.iter()
                                    .find(|t| theme_slug(&t.name) == *slug)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| slug.clone());
                                let slug_remove = slug.clone();
                                let slug_link = slug.clone();
                                rsx! {
                                    div { class: "sidebar-shortlist-item",
                                        Link {
                                            to: Route::ThemeDetail { slug: slug_link },
                                            class: "sidebar-shortlist-name-link",
                                            onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                            span { class: "sidebar-shortlist-name", "{name}" }
                                        }
                                        button {
                                            class: "sidebar-shortlist-remove",
                                            title: "Remove from shortlist",
                                            onclick: move |_| {
                                                shortlist.write().0.retain(|s| s != &slug_remove);
                                            },
                                            "\u{00d7}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "sidebar-shortlist-actions",
                        if sl_count > 0 {
                            button {
                                class: "sidebar-clear-btn",
                                onclick: move |_| {
                                    shortlist.write().0.clear();
                                },
                                "Clear"
                            }
                        }
                    }
                }
            }

            // CVD (pinned to bottom)
            div { class: "sidebar-section sidebar-cvd",
                span { class: "sidebar-section-label",
                    title: "Simulate color vision deficiency",
                    "CVD"
                }
                FilterButton {
                    label: "Normal",
                    active: cvd.is_none(),
                    onclick: move |_| cvd_sim.set(CvdSimulation(None)),
                }
                for cvd_type in litmus_model::cvd::CvdType::all() {
                    {
                        let ct = *cvd_type;
                        let label = ct.label();
                        let desc = ct.description();
                        rsx! {
                            button {
                                class: if cvd == Some(ct) { "filter-btn filter-btn-active" } else { "filter-btn" },
                                aria_pressed: if cvd == Some(ct) { "true" } else { "false" },
                                title: "{desc}",
                                onclick: move |_| cvd_sim.set(CvdSimulation(Some(ct))),
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
