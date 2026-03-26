use dioxus::prelude::*;

use crate::components::*;
use crate::fixtures;
use crate::state::*;
use crate::term_renderer;
use crate::themes;
use crate::Route;

/// Fixture-centric view: one fixture rendered across all themes in a grid.
#[component]
pub fn SceneAcrossThemes(provider: String, scene_id: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let all_themes = themes::themes_for_provider(&active_provider.read().0);
    let all_fixtures = fixtures::all_fixtures();
    let fixture = all_fixtures.iter().find(|f| f.id == scene_id);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;

    match fixture {
        Some(fixture) => {
            rsx! {
                div { class: "page-scene-across",
                    h2 { class: "page-title", "{fixture.name}" }

                    // Fixture selector tabs
                    div { class: "scene-tabs scene-tabs-margin",
                        for f in all_fixtures {
                            Link {
                                to: Route::SceneAcrossThemes { provider: provider.clone(), scene_id: f.id.clone() },
                                class: if f.id == fixture.id { "scene-tab scene-tab-active" } else { "scene-tab" },
                                "{f.name}"
                            }
                        }
                    }

                    div { class: "scene-grid",
                        for theme in &all_themes {
                            {
                                let sim_theme = maybe_simulate(theme, cvd);
                                rsx! {
                                    div { class: "scene-grid-card",
                                        div { class: "scene-grid-card-header",
                                            Link {
                                                to: Route::ThemeDetail {
                                                    provider: provider.clone(),
                                                    slug: theme_slug(&theme.name),
                                                },
                                                class: "accent-link scene-grid-theme-name",
                                                "{theme.name}"
                                            }
                                            ShortlistToggle {
                                                slug: theme_slug(&theme.name),
                                                name: theme.name.clone(),
                                            }
                                        }
                                        term_renderer::TermOutputView {
                                            theme: sim_theme,
                                            output: fixture.clone(),
                                            compact: true,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div {
                    h2 { "Fixture not found" }
                    p { "No fixture matches \"{scene_id}\"." }
                    Link { to: Route::ThemeList { provider: provider.clone() }, class: "accent-link", "Back to all themes" }
                }
            }
        }
    }
}
