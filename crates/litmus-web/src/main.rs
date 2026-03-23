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

/// Production screenshot manifest URL.
const MANIFEST_CDN_URL: &str = "https://screenshots.litmus.edger.dev/manifest.json";

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Shortlist::default()));
    use_context_provider(|| Signal::new(CvdSimulation::default()));
    use_context_provider(|| Signal::new(ActiveScene::default()));
    use_context_provider(|| Signal::new(AppThemeSlug::default()));
    use_context_provider(|| Signal::new(SidebarOpen::default()));
    use_context_provider(|| Signal::new(VisibleScenes::default()));
    use_context_provider(|| Signal::new(SceneIssueCounts::default()));
    use_context_provider(|| Signal::new(ManifestState::default()));

    // Fetch screenshot manifest on app load.
    // In dev: derive screenshot server URL from current origin (port 8883 → 8884).
    // In production: fetch from CDN.
    let mut manifest_state = use_context::<Signal<ManifestState>>();
    use_effect(move || {
        let cdn_url = MANIFEST_CDN_URL;
        let js = format!(
            r#"
            async function tryFetch(url) {{
                try {{
                    const resp = await fetch(url);
                    if (resp.ok) return await resp.text();
                }} catch(e) {{}}
                return null;
            }}
            // In dev, screenshot server runs on port 8884 (web app on 8883).
            // Derive the URL from current origin so it works behind reverse proxies.
            const origin = window.location.origin;
            if (origin.includes("8883")) {{
                const screenshotOrigin = origin.replace("8883", "8884");
                const local = await tryFetch(screenshotOrigin + "/manifest.json");
                if (local) {{
                    // Rewrite base_url to match the actual screenshot server origin
                    const parsed = JSON.parse(local);
                    parsed.base_url = screenshotOrigin;
                    return JSON.stringify(parsed);
                }}
            }}
            return await tryFetch("{cdn_url}");
            "#
        );
        spawn(async move {
            use dioxus::document::eval;
            if let Ok(val) = eval(&js).await
                && let Some(text) = val.as_str()
                && let Ok(manifest) = serde_json::from_str(text)
            {
                manifest_state.set(ManifestState(Some(manifest)));
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/style.css") }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: asset!("assets/litmus-icon.svg") }
        Router::<Route> {}
    }
}
