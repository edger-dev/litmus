use dioxus::prelude::*;

use crate::components::{FavoritesCheckbox, GitHubStars, SceneMinimap};
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


/// Persistent left sidebar: navigation, favorites management, CVD toggle.
#[component]
pub fn Sidebar() -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let provider = active_provider.read().0.clone();
    let all_themes = themes::themes_for_provider(&provider);
    let providers = themes::available_providers();
    let mut favorites = use_context::<Signal<Favorites>>();
    let mut cvd_sim = use_context::<Signal<CvdSimulation>>();
    let mut sidebar_open = use_context::<Signal<SidebarOpen>>();
    let app_theme = use_context::<Signal<AppThemeSlug>>();
    let mut alert = use_context::<Signal<AlertMessage>>();
    let nav = navigator();
    let current_route = use_route::<Route>();

    let cvd = cvd_sim.read().0;
    let sl = favorites.read().clone();
    let sl_count = sl.0.len();
    let app_slug = app_theme.read().0.clone();

    let show_favorites = sl_count > 0 || app_slug.is_some();

    // Collect all theme slugs for the "Feel Lucky" random pick
    let all_slugs: Vec<String> = all_themes.iter().map(|t| theme_slug(&t.name)).collect();

    // Determine which nav item is active based on current route
    let is_browse_active = matches!(current_route, Route::ThemeList { .. });
    let is_compare_active = matches!(current_route, Route::CompareThemes { .. } | Route::SceneAcrossThemes { .. });
    let detail_slug = match &current_route {
        Route::ThemeDetail { slug, .. } => Some(slug.clone()),
        _ => None,
    };
    let compare_slugs: Vec<String> = match &current_route {
        Route::CompareThemes { slugs, .. } => slugs
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect(),
        _ => Vec::new(),
    };

    // History: last visited non-favorite detail themes
    let visit_history = use_context::<Signal<VisitHistory>>();
    let history_items: Vec<(String, String)> = {
        let history = visit_history.read();
        let fav_set: std::collections::HashSet<&str> = sl.0.iter().map(|s| s.as_str()).collect();
        history.0.iter()
            .filter(|s| !fav_set.contains(s.as_str()) && app_slug.as_deref() != Some(s.as_str()))
            .take(MAX_HISTORY)
            .map(|slug| {
                let name = all_themes.iter()
                    .find(|t| theme_slug(&t.name) == *slug)
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| slug.clone());
                (slug.clone(), name)
            })
            .collect()
    };
    let show_history = !history_items.is_empty();

    // "vs." button: show last compared theme name, or "Feel Lucky" if none
    let last_compared = use_context::<Signal<LastComparedSlug>>();
    let vs_label: String = match last_compared.read().0.as_deref() {
        Some(slug) => {
            let name = all_themes.iter()
                .find(|t| theme_slug(&t.name) == slug)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| slug.to_string());
            format!("vs. {name}")
        }
        None => "Feel Lucky".to_string(),
    };

    rsx! {
        aside {
            class: "sidebar",
            role: "navigation",
            aria_label: "Main navigation",

            // Header
            div { class: "sidebar-header",
                div { class: "sidebar-header-row",
                    Link { to: Route::ThemeList { provider: provider.clone() }, class: "sidebar-logo",
                        img {
                            class: "sidebar-logo-icon",
                            src: asset!("assets/litmus-icon.svg"),
                            alt: "Litmus",
                            width: "24",
                            height: "24",
                        }
                        span { class: "sidebar-logo-text", "Litmus" }
                    }
                    GitHubStars {}
                }
                span { class: "sidebar-subtitle", "terminal color theme previewer" }
            }

            // Provider selector
            div { class: "sidebar-section sidebar-provider",
                div { class: "sidebar-provider-buttons",
                    for p in providers.iter() {
                        {
                            let is_active = *p == provider;
                            let p_name = p.clone();
                            // Check if current theme(s) are available in target provider
                            let detail_slug_for_check = detail_slug.clone();
                            let compare_slugs_for_check = compare_slugs.clone();
                            let missing_names: Vec<String> = if let Some(slug) = detail_slug_for_check.as_ref() {
                                if themes::theme_available_for_provider(slug, &p_name) {
                                    Vec::new()
                                } else {
                                    let name = all_themes.iter()
                                        .find(|t| theme_slug(&t.name) == *slug)
                                        .map(|t| t.name.clone())
                                        .unwrap_or_else(|| slug.clone());
                                    vec![name]
                                }
                            } else {
                                compare_slugs_for_check.iter().filter_map(|slug| {
                                    if themes::theme_available_for_provider(slug, &p_name) {
                                        None
                                    } else {
                                        // Use current provider's theme list for the display name
                                        Some(all_themes.iter()
                                            .find(|t| theme_slug(&t.name) == *slug)
                                            .map(|t| t.name.clone())
                                            .unwrap_or_else(|| slug.clone()))
                                    }
                                }).collect()
                            };
                            let is_available = missing_names.is_empty();
                            let btn_class = if is_active {
                                "provider-btn provider-btn-active"
                            } else if !is_available {
                                "provider-btn provider-btn-unavailable"
                            } else {
                                "provider-btn"
                            };
                            let new_route = current_route.with_provider(&p_name);
                            if is_available {
                                rsx! {
                                    Link {
                                        to: new_route,
                                        class: "{btn_class}",
                                        "{p_name}"
                                    }
                                }
                            } else {
                                rsx! {
                                    button {
                                        class: "{btn_class}",
                                        onclick: {
                                            let missing = missing_names.join(", ");
                                            move |_| {
                                                let msg = format!("{missing} not available for {p_name}");
                                                alert.set(AlertMessage(Some(msg)));
                                                // Auto-dismiss after 3s
                                                spawn(async move {
                                                    use dioxus::document::eval;
                                                    let _ = eval("await new Promise(r => setTimeout(r, 3000))").await;
                                                    alert.set(AlertMessage(None));
                                                });
                                            }
                                        },
                                        "{p_name}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Nav links
            div { class: "sidebar-section sidebar-nav",
                Link {
                    to: Route::ThemeList { provider: provider.clone() },
                    class: if is_browse_active { "sidebar-nav-link active" } else { "sidebar-nav-link" },
                    onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                    "Browse Themes"
                }
                if is_compare_active {
                    {
                        let label = {
                            let left = compare_slugs.first().map(|s| {
                                all_themes.iter().find(|t| theme_slug(&t.name) == *s)
                                    .map(|t| t.name.clone()).unwrap_or_else(|| s.clone())
                            });
                            let right = compare_slugs.get(1).map(|s| {
                                all_themes.iter().find(|t| theme_slug(&t.name) == *s)
                                    .map(|t| t.name.clone()).unwrap_or_else(|| s.clone())
                            });
                            let left_is_current = compare_slugs.first()
                                .map(|s| app_slug.as_deref() == Some(s.as_str()))
                                .unwrap_or(false);
                            match (left, right, left_is_current) {
                                (_, Some(r), true) => format!("vs {r}"),
                                (Some(l), Some(r), false) => format!("{l} vs {r}"),
                                _ => "Side by Side".to_string(),
                            }
                        };
                        rsx! {
                            span {
                                class: "sidebar-nav-link active",
                                "{label}"
                            }
                        }
                    }
                } else {
                    button {
                        class: "sidebar-nav-link sidebar-feel-lucky",
                        onclick: move |_| {
                            sidebar_open.set(SidebarOpen(false));
                            let current = app_theme.read().0.clone();
                            // If we have a last-compared slug, go directly there
                            if let Some(ref last) = last_compared.read().0 {
                                let app_s = current.as_deref().unwrap_or("tokyo-night");
                                let p = active_provider.read().0.clone();
                                nav.push(Route::CompareThemes {
                                    provider: p,
                                    slugs: format!("{},{}", app_s, last),
                                });
                                return;
                            }
                            // Otherwise random "Feel Lucky"
                            if all_slugs.len() >= 2 {
                                let candidates: Vec<&String> = all_slugs.iter()
                                    .filter(|s| current.as_deref() != Some(s.as_str()))
                                    .collect();
                                if candidates.is_empty() { return; }
                                let idx = random_index(candidates.len());
                                let random_slug = candidates[idx].clone();

                                let compare = if let Some(ref cur) = current {
                                    format!("{},{}", cur, random_slug)
                                } else {
                                    let idx2 = (idx + 1 + random_index(candidates.len().saturating_sub(1).max(1))) % candidates.len();
                                    let random_slug2 = candidates[idx2].clone();
                                    format!("{},{}", random_slug, random_slug2)
                                };
                                let p = active_provider.read().0.clone();
                                nav.push(Route::CompareThemes { provider: p, slugs: compare });
                            }
                        },
                        "{vs_label}"
                    }
                }
            }

            // Favorites
            if show_favorites {
                div { class: "sidebar-section sidebar-favorites",
                    div { class: "sidebar-section-label", "Favorites ({sl_count})" }
                    div { class: "sidebar-favorites-items",
                        // Pinned app theme entry
                        if let Some(ref app_s) = app_slug {
                            {
                                let name = all_themes.iter()
                                    .find(|t| theme_slug(&t.name) == *app_s)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| app_s.clone());
                                let is_viewing = detail_slug.as_deref() == Some(app_s.as_str());
                                rsx! {
                                    div {
                                        class: if is_viewing { "sidebar-favorites-item sidebar-favorites-current sidebar-favorites-viewing" } else { "sidebar-favorites-item sidebar-favorites-current" },
                                        Link {
                                            to: Route::ThemeDetail { provider: provider.clone(), slug: app_s.clone() },
                                            class: "sidebar-favorites-name-link",
                                            onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                            span { class: "sidebar-favorites-name", "{name}" }
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
                                let is_viewing = detail_slug.as_deref() == Some(slug.as_str());
                                let slug_link = slug.clone();
                                let slug_vs = slug.clone();
                                rsx! {
                                    div {
                                        class: if is_viewing { "sidebar-favorites-item sidebar-favorites-viewing" } else { "sidebar-favorites-item" },
                                        Link {
                                            to: Route::ThemeDetail { provider: provider.clone(), slug: slug_link },
                                            class: "sidebar-favorites-name-link",
                                            onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                            span { class: "sidebar-favorites-name", "{name}" }
                                        }
                                        span { class: "sidebar-item-actions",
                                            Link {
                                                class: "sidebar-vs-btn",
                                                to: Route::CompareThemes {
                                                    provider: provider.clone(),
                                                    slugs: format!("{},{}", app_slug.as_deref().unwrap_or("tokyo-night"), slug_vs),
                                                },
                                                onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                                "vs"
                                            }
                                            FavoritesCheckbox { slug: slug.clone(), name: name.clone() }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "sidebar-favorites-actions",
                        if sl_count > 0 {
                            button {
                                class: "sidebar-clear-btn",
                                onclick: move |_| {
                                    favorites.write().0.clear();
                                },
                                "Clear"
                            }
                        }
                    }
                }
            }

            // History: last visited non-favorite detail themes
            if show_history {
                div { class: "sidebar-section sidebar-history",
                    div { class: "sidebar-section-label", "History" }
                    div { class: "sidebar-favorites-items",
                        for (slug, name) in &history_items {
                            {
                                let is_viewing = detail_slug.as_deref() == Some(slug.as_str());
                                let slug_link = slug.clone();
                                let slug_vs = slug.clone();
                                let slug_fav = slug.clone();
                                let name_fav = name.clone();
                                rsx! {
                                    div {
                                        class: if is_viewing { "sidebar-favorites-item sidebar-favorites-viewing" } else { "sidebar-favorites-item" },
                                        Link {
                                            to: Route::ThemeDetail { provider: provider.clone(), slug: slug_link },
                                            class: "sidebar-favorites-name-link",
                                            onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                            span { class: "sidebar-favorites-name", "{name}" }
                                        }
                                        span { class: "sidebar-item-actions",
                                            Link {
                                                class: "sidebar-vs-btn",
                                                to: Route::CompareThemes {
                                                    provider: provider.clone(),
                                                    slugs: format!("{},{}", app_slug.as_deref().unwrap_or("tokyo-night"), slug_vs),
                                                },
                                                onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                                "vs"
                                            }
                                            FavoritesCheckbox { slug: slug_fav, name: name_fav }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Scene minimap (shown on detail + compare pages)
            if matches!(current_route, Route::ThemeDetail { .. } | Route::CompareThemes { .. }) {
                div { class: "sidebar-section",
                    div { class: "sidebar-section-label", "Fixtures" }
                    SceneMinimap {
                        items: crate::fixtures::all_fixtures()
                            .iter()
                            .map(|f| (f.id.clone(), f.name.clone()))
                            .collect(),
                        show_badges: true,
                    }
                }
            }

            // CVD (pinned to bottom)
            div { class: "sidebar-section sidebar-cvd",
                label { class: "sidebar-section-label",
                    title: "Simulate color vision deficiency",
                    r#for: "cvd-select",
                    "CVD"
                }
                select {
                    id: "cvd-select",
                    class: "sidebar-cvd-select",
                    onchange: move |evt: Event<FormData>| {
                        let val = evt.value();
                        let sim = match val.as_str() {
                            "protanopia" => Some(litmus_model::cvd::CvdType::Protanopia),
                            "deuteranopia" => Some(litmus_model::cvd::CvdType::Deuteranopia),
                            "tritanopia" => Some(litmus_model::cvd::CvdType::Tritanopia),
                            _ => None,
                        };
                        cvd_sim.set(CvdSimulation(sim));
                    },
                    option {
                        value: "normal",
                        selected: cvd.is_none(),
                        "Normal Vision"
                    }
                    for cvd_type in litmus_model::cvd::CvdType::all() {
                        {
                            let ct = *cvd_type;
                            let label = ct.label();
                            let desc = ct.description();
                            let val = match ct {
                                litmus_model::cvd::CvdType::Protanopia => "protanopia",
                                litmus_model::cvd::CvdType::Deuteranopia => "deuteranopia",
                                litmus_model::cvd::CvdType::Tritanopia => "tritanopia",
                            };
                            rsx! {
                                option {
                                    value: val,
                                    selected: cvd == Some(ct),
                                    title: "{desc}",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
