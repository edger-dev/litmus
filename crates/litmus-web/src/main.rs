mod components;
mod family;
mod pages;
mod scene_renderer;
mod screenshot_view;
mod shell;
mod sidebar;
pub mod state;
mod themes;

use dioxus::prelude::*;

use pages::*;
use shell::Shell;
use state::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    ThemeList {},
    #[route("/theme/:slug")]
    ThemeDetail { slug: String },
    #[route("/scene/:scene_id")]
    SceneAcrossThemes { scene_id: String },
    #[route("/compare/:slugs")]
    CompareThemes { slugs: String },
}

/// CDN base URL for the screenshot manifest. Empty string disables screenshot fetching.
const MANIFEST_URL: &str = "https://screenshots.litmus.edger.dev/manifest.json";

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Shortlist::default()));
    use_context_provider(|| Signal::new(CvdSimulation::default()));
    use_context_provider(|| Signal::new(ActiveScene::default()));
    use_context_provider(|| Signal::new(AppThemeSlug::default()));
    use_context_provider(|| Signal::new(SidebarOpen::default()));
    use_context_provider(|| Signal::new(VisibleScenes::default()));
    use_context_provider(|| Signal::new(SceneIssueCounts::default()));
    use_context_provider(|| Signal::new(ActiveProvider::default()));
    use_context_provider(|| Signal::new(ManifestState::default()));

    // Fetch screenshot manifest from CDN on app load
    let mut manifest_state = use_context::<Signal<ManifestState>>();
    use_effect(move || {
        if MANIFEST_URL.is_empty() {
            return;
        }
        let url = MANIFEST_URL;
        let js = format!(
            r#"
            try {{
                const resp = await fetch("{url}");
                if (!resp.ok) return null;
                return await resp.text();
            }} catch(e) {{
                return null;
            }}
            "#
        );
        spawn(async move {
            use dioxus::document::eval;
            if let Ok(val) = eval(&js).await {
                if let Some(text) = val.as_str() {
                    if let Ok(manifest) = serde_json::from_str(text) {
                        manifest_state.set(ManifestState(Some(manifest)));
                    }
                }
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/style.css") }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: asset!("assets/litmus-icon.svg") }
        Router::<Route> {}
    }
}
