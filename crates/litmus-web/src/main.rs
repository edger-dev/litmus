mod components;
mod family;
mod pages;
mod scene_renderer;
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

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(Shortlist::default()));
    use_context_provider(|| Signal::new(CvdSimulation::default()));
    use_context_provider(|| Signal::new(ActiveScene::default()));
    use_context_provider(|| Signal::new(AppThemeSlug::default()));
    use_context_provider(|| Signal::new(SidebarOpen::default()));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("assets/style.css") }
        document::Link { rel: "icon", r#type: "image/svg+xml", href: asset!("assets/litmus-icon.svg") }
        Router::<Route> {}
    }
}
