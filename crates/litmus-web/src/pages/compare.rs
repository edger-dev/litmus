use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Multi-theme comparison (2-4 themes side by side).
#[component]
pub fn CompareThemes(slugs: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;

    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).collect();

    let compare_themes: Vec<litmus_model::Theme> = slug_list
        .iter()
        .filter_map(|slug| all_themes.iter().find(|t| theme_slug(&t.name) == *slug).cloned())
        .map(|t| maybe_simulate(&t, cvd))
        .collect();

    if compare_themes.is_empty() {
        return rsx! {
            div {
                h2 { "No themes found" }
                p { "Could not find any matching themes." }
                Link { to: Route::ThemeList {}, class: "accent-link", "Back to all themes" }
            }
        };
    }

    let n = compare_themes.len();
    let title = compare_themes
        .iter()
        .map(|t| t.name.as_str())
        .collect::<Vec<_>>()
        .join(" vs ");
    let grid_cols = format!("repeat({n}, 1fr)");

    rsx! {
        div { class: "page-compare",
            h2 { class: "page-title", "{title}" }

            if compare_themes.len() >= 2 {
                ColorDiffTable { themes: compare_themes.clone() }
            }

            for scene in &scenes {
                div { class: "compare-scene-group",
                    id: "scene-{scene.id}",
                    h3 { class: "compare-scene-name", "{scene.name}" }

                    div {
                        class: "compare-grid",
                        style: "--compare-cols: {grid_cols};",

                        for theme in &compare_themes {
                            div { class: "compare-grid-item",
                                div { class: "compare-grid-item-header",
                                    span { class: "compare-grid-item-name", "{theme.name}" }
                                    Link {
                                        to: Route::ThemeDetail {
                                            slug: theme_slug(&theme.name),
                                        },
                                        class: "text-success choose-link",
                                        "Choose"
                                    }
                                }
                                scene_renderer::SceneView {
                                    theme: theme.clone(),
                                    scene: scene.clone(),
                                    compact: n > 2,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
