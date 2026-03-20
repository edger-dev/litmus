use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            h1 { "litmus" }
            p { "Terminal color theme previewer — coming soon." }
        }
    }
}
