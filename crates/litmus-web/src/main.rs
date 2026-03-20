mod scene_renderer;
mod themes;

use dioxus::prelude::*;
use scene_renderer::AllScenesView;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let all_themes = use_memo(|| themes::load_embedded_themes());
    let mut selected_idx = use_signal(|| 0usize);

    let themes = &*all_themes.read();
    if themes.is_empty() {
        return rsx! { p { "No themes found." } };
    }

    let idx = (*selected_idx.read()).min(themes.len() - 1);
    let current_theme = themes[idx].clone();
    let bg = current_theme.background.to_hex();
    let fg = current_theme.foreground.to_hex();

    rsx! {
        div {
            style: "min-height: 100vh; background-color: {bg}; color: {fg}; \
                    font-family: system-ui, sans-serif; padding: 2rem;",

            // Header
            div {
                style: "max-width: 900px; margin: 0 auto;",

                h1 {
                    style: "font-size: 1.5rem; margin-bottom: 1rem;",
                    "litmus"
                }

                // Theme selector
                div {
                    style: "margin-bottom: 2rem; display: flex; align-items: center; gap: 1rem;",

                    label {
                        style: "font-weight: bold;",
                        "Theme:"
                    }

                    select {
                        style: "background: {bg}; color: {fg}; border: 1px solid {fg}; \
                                padding: 0.4rem 0.6rem; border-radius: 0.25rem; \
                                font-size: 1rem;",
                        value: "{idx}",
                        onchange: move |evt: Event<FormData>| {
                            if let Ok(i) = evt.value().parse::<usize>() {
                                selected_idx.set(i);
                            }
                        },
                        for (i, t) in themes.iter().enumerate() {
                            option { value: "{i}", "{t.name}" }
                        }
                    }
                }

                // Scenes
                AllScenesView { theme: current_theme }
            }
        }
    }
}
