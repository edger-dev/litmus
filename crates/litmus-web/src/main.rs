mod components;
mod family;
mod fixtures;
mod pages;
mod screenshot_view;
mod shell;
mod sidebar;
pub mod state;
mod term_renderer;
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
    #[nest("/:provider")]
        #[route("/")]
        ThemeList { provider: String },
        #[route("/theme/:slug")]
        ThemeDetail { provider: String, slug: String },
        #[route("/scene/:scene_id")]
        SceneAcrossThemes { provider: String, scene_id: String },
        #[route("/compare/:slugs")]
        CompareThemes { provider: String, slugs: String },
    #[end_nest]
    #[end_layout]
    #[route("/")]
    Root {},
}

/// Redirect from bare `/` to the default provider.
#[component]
fn Root() -> Element {
    let nav = navigator();
    let default_provider = themes::available_providers().first().cloned().unwrap_or("kitty".into());
    nav.replace(Route::ThemeList { provider: default_provider });
    rsx! {}
}

impl Route {
    /// Extract provider from the current route, if any.
    pub fn provider(&self) -> Option<&str> {
        match self {
            Route::ThemeList { provider } => Some(provider),
            Route::ThemeDetail { provider, .. } => Some(provider),
            Route::SceneAcrossThemes { provider, .. } => Some(provider),
            Route::CompareThemes { provider, .. } => Some(provider),
            Route::Root {} => None,
        }
    }

    /// Return same route with a different provider.
    pub fn with_provider(&self, new_provider: &str) -> Route {
        match self {
            Route::ThemeList { .. } => Route::ThemeList { provider: new_provider.into() },
            Route::ThemeDetail { slug, .. } => Route::ThemeDetail { provider: new_provider.into(), slug: slug.clone() },
            Route::SceneAcrossThemes { scene_id, .. } => Route::SceneAcrossThemes { provider: new_provider.into(), scene_id: scene_id.clone() },
            Route::CompareThemes { slugs, .. } => Route::CompareThemes { provider: new_provider.into(), slugs: slugs.clone() },
            Route::Root {} => Route::ThemeList { provider: new_provider.into() },
        }
    }
}

/// Production screenshot manifest URL.
const MANIFEST_CDN_URL: &str = "https://screenshots.litmus.edger.dev/manifest.json";

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Shortlist::default()));
    use_context_provider(|| Signal::new(CvdSimulation::default()));
    use_context_provider(|| Signal::new(AppThemeSlug::default()));
    use_context_provider(|| Signal::new(SidebarOpen::default()));
    use_context_provider(|| Signal::new(VisibleScenes::default()));
    use_context_provider(|| Signal::new(SceneIssueCounts::default()));
    use_context_provider(|| Signal::new(CompareIssueDots::default()));
    use_context_provider(|| Signal::new(ManifestState::default()));
    use_context_provider(|| Signal::new(ActiveProvider::default()));
    use_context_provider(|| Signal::new(AlertMessage::default()));

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
                    const resp = await fetch(url, {{ cache: 'no-cache' }});
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
