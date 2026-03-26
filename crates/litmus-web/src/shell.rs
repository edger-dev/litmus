use dioxus::prelude::*;
use dioxus::document::eval;

use crate::sidebar::Sidebar;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Generate JS to set CSS custom properties from a theme.
fn theme_to_css_vars_js(theme: &litmus_model::Theme) -> String {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let accent = theme.ansi.blue.to_hex();
    let success = theme.ansi.green.to_hex();
    let warning = theme.ansi.yellow.to_hex();
    let error = theme.ansi.red.to_hex();
    let is_light = litmus_model::contrast::relative_luminance(&theme.background) > 0.5;

    // For sidebar bg: slightly shift the background
    // For dark themes: lighten slightly; for light themes: darken slightly
    let sidebar_bg = if is_light {
        // darken by mixing with black
        format!(
            "color-mix(in srgb, {} 92%, black)",
            bg
        )
    } else {
        // lighten by mixing with white
        format!(
            "color-mix(in srgb, {} 92%, white)",
            bg
        )
    };

    let data_theme = if is_light { "light" } else { "dark" };

    format!(
        r#"(function() {{
  var s = document.documentElement.style;
  s.setProperty('--app-bg', '{bg}');
  s.setProperty('--app-fg', '{fg}');
  s.setProperty('--app-accent', '{accent}');
  s.setProperty('--app-success', '{success}');
  s.setProperty('--app-warning', '{warning}');
  s.setProperty('--app-error', '{error}');
  s.setProperty('--app-border', '{fg}26');
  s.setProperty('--app-surface', '{fg}0d');
  s.setProperty('--app-sidebar-bg', '{sidebar_bg}');
  s.setProperty('--app-muted', '{fg}99');
  s.setProperty('--app-hover', '{accent}1a');
  s.setProperty('--app-active', '{accent}33');
  document.documentElement.setAttribute('data-theme', '{data_theme}');
}})();"#
    )
}

/// JS to reset to default Tokyo Night-inspired theme.
fn default_theme_js() -> &'static str {
    r#"(function() {
  var s = document.documentElement.style;
  s.setProperty('--app-bg', '#1a1b26');
  s.setProperty('--app-fg', '#c0caf5');
  s.setProperty('--app-accent', '#7aa2f7');
  s.setProperty('--app-success', '#a6e3a1');
  s.setProperty('--app-warning', '#e0af68');
  s.setProperty('--app-error', '#f38ba8');
  s.setProperty('--app-border', '#c0caf526');
  s.setProperty('--app-surface', '#c0caf50d');
  s.setProperty('--app-sidebar-bg', 'color-mix(in srgb, #1a1b26 92%, white)');
  s.setProperty('--app-muted', '#c0caf599');
  s.setProperty('--app-hover', '#7aa2f71a');
  s.setProperty('--app-active', '#7aa2f733');
  document.documentElement.setAttribute('data-theme', 'dark');
})();"#
}

/// Shell layout: sidebar + main content area.
#[component]
pub fn Shell() -> Element {
    let app_theme = use_context::<Signal<AppThemeSlug>>();
    let sidebar_open = use_context::<Signal<SidebarOpen>>();
    let is_open = sidebar_open.read().0;

    let mut active_provider = use_context::<Signal<ActiveProvider>>();

    // Sync URL provider → ActiveProvider signal
    let current_route = use_route::<Route>();
    if let Some(url_provider) = current_route.provider()
        && active_provider.read().0 != url_provider
    {
        active_provider.set(ActiveProvider(url_provider.to_string()));
    }

    // Reveal body once WASM has mounted (prevents flash of unstyled content)
    use_effect(|| {
        eval("document.body.classList.add('app-ready');");
    });

    // Apply app theme via CSS custom properties
    use_effect(move || {
        let slug = app_theme.read().0.clone();
        let provider = active_provider.read().0.clone();
        let js = match &slug {
            Some(s) => {
                let all_themes = themes::themes_for_provider(&provider);
                if let Some(theme) = all_themes.iter().find(|t| theme_slug(&t.name) == *s) {
                    theme_to_css_vars_js(theme)
                } else {
                    default_theme_js().to_string()
                }
            }
            None => default_theme_js().to_string(),
        };
        eval(&js);
    });

    let mut sidebar_open_write = sidebar_open;

    rsx! {
        div { class: "app-layout",
            a { class: "skip-to-content", href: "#main-content", "Skip to content" }

            // Mobile header
            div { class: "mobile-header",
                button {
                    class: "hamburger-btn",
                    aria_label: "Toggle navigation",
                    onclick: move |_| {
                        let open = sidebar_open.read().0;
                        sidebar_open_write.set(SidebarOpen(!open));
                    },
                    span { class: "hamburger-icon" }
                }
                span { class: "mobile-logo", "litmus" }
            }

            // Sidebar overlay (mobile)
            if is_open {
                div {
                    class: "sidebar-overlay",
                    onclick: move |_| sidebar_open_write.set(SidebarOpen(false)),
                }
            }

            // Sidebar
            div {
                class: if is_open { "sidebar-wrapper sidebar-open" } else { "sidebar-wrapper" },
                Sidebar {}
            }

            // Main content
            main {
                id: "main-content",
                class: "main-content",
                AlertBanner {}
                Outlet::<Route> {}
            }
        }
    }
}

/// Dismissible alert banner, shown when provider switch is blocked.
#[component]
fn AlertBanner() -> Element {
    let mut alert = use_context::<Signal<AlertMessage>>();
    let msg = alert.read().0.clone();

    match msg {
        Some(text) => rsx! {
            div {
                class: "alert-banner",
                span { "{text}" }
                button {
                    class: "alert-banner-dismiss",
                    onclick: move |_| alert.set(AlertMessage(None)),
                    "\u{00d7}"
                }
            }
        },
        None => rsx! {},
    }
}
