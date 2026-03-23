use litmus_model::Theme;
use std::path::Path;

use super::{ProviderCapture, TermGeometry};

pub struct WeztermProvider;

impl ProviderCapture for WeztermProvider {
    fn slug(&self) -> &str {
        "wezterm"
    }

    fn name(&self) -> &str {
        "WezTerm"
    }

    fn config_extension(&self) -> &str {
        "lua"
    }

    fn generate_config(&self, theme: &Theme, geometry: &TermGeometry) -> String {
        let ansi = theme.ansi.as_array();
        let hex = |c: &litmus_model::Color| c.to_hex();

        let ansi_regular: Vec<String> = ansi[..8].iter().map(|c| format!("\"{}\"", hex(c))).collect();
        let ansi_bright: Vec<String> = ansi[8..].iter().map(|c| format!("\"{}\"", hex(c))).collect();

        format!(
            r#"local wezterm = require 'wezterm'
local config = wezterm.config_builder()

config.colors = {{
  background = "{bg}",
  foreground = "{fg}",
  cursor_bg = "{cursor}",
  cursor_fg = "{bg}",
  selection_bg = "{sel_bg}",
  selection_fg = "{sel_fg}",
  ansi = {{ {ansi_regular} }},
  brights = {{ {ansi_bright} }},
}}

config.font = wezterm.font("{font_family}")
config.font_size = {font_size:.1}
config.initial_cols = {cols}
config.initial_rows = {rows}

-- Screenshot-specific settings
config.enable_wayland = true
config.hide_tab_bar_if_only_one_tab = true
config.window_decorations = "NONE"
config.window_padding = {{ left = 0, right = 0, top = 0, bottom = 0 }}
config.enable_scroll_bar = false
config.audible_bell = "Disabled"
config.check_for_updates = false
config.default_cursor_style = "SteadyBlock"
config.animation_fps = 1

return config
"#,
            bg = hex(&theme.background),
            fg = hex(&theme.foreground),
            cursor = hex(&theme.cursor),
            sel_bg = hex(&theme.selection_background),
            sel_fg = hex(&theme.selection_foreground),
            ansi_regular = ansi_regular.join(", "),
            ansi_bright = ansi_bright.join(", "),
            font_family = geometry.font_family,
            font_size = geometry.font_size,
            cols = geometry.cols,
            rows = geometry.rows,
        )
    }

    fn build_launch_args(&self, config_path: &Path, command: &str) -> Vec<String> {
        vec![
            "wezterm".to_string(),
            "start".to_string(),
            "--config-file".to_string(),
            config_path.to_string_lossy().into_owned(),
            "--".to_string(),
            "bash".to_string(),
            "-c".to_string(),
            command.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use litmus_model::{AnsiColors, Color, Theme};

    fn sample_theme() -> Theme {
        let c = |r, g, b| Color::new(r, g, b);
        Theme {
            name: "Test Theme".to_string(),
            background: c(30, 30, 46),
            foreground: c(205, 214, 244),
            cursor: c(243, 166, 197),
            selection_background: c(88, 91, 112),
            selection_foreground: c(205, 214, 244),
            ansi: AnsiColors::from_array([
                c(69, 71, 90),    // black
                c(243, 139, 168), // red
                c(166, 227, 161), // green
                c(249, 226, 175), // yellow
                c(137, 180, 250), // blue
                c(245, 194, 231), // magenta
                c(148, 226, 213), // cyan
                c(186, 194, 222), // white
                c(88, 91, 112),   // bright black
                c(255, 142, 168), // bright red
                c(171, 232, 166), // bright green
                c(254, 231, 181), // bright yellow
                c(140, 183, 255), // bright blue
                c(247, 199, 233), // bright magenta
                c(151, 229, 215), // bright cyan
                c(228, 228, 228), // bright white
            ]),
        }
    }

    #[test]
    fn config_contains_theme_colors() {
        let provider = WeztermProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("background = \"#1E1E2E\""));
        assert!(config.contains("foreground = \"#CDD6F4\""));
        assert!(config.contains("\"#45475A\""));
    }

    #[test]
    fn config_contains_geometry() {
        let provider = WeztermProvider;
        let theme = sample_theme();
        let geometry = TermGeometry {
            cols: 80,
            rows: 24,
            font_size: 12.0,
            font_family: "FiraCode".to_string(),
        };
        let config = provider.generate_config(&theme, &geometry);

        assert!(config.contains("wezterm.font(\"FiraCode\")"));
        assert!(config.contains("font_size = 12.0"));
        assert!(config.contains("initial_cols = 80"));
        assert!(config.contains("initial_rows = 24"));
    }

    #[test]
    fn config_has_screenshot_settings() {
        let provider = WeztermProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("enable_wayland = true"));
        assert!(config.contains("window_decorations = \"NONE\""));
        assert!(config.contains("hide_tab_bar_if_only_one_tab = true"));
        assert!(config.contains("default_cursor_style = \"SteadyBlock\""));
    }

    #[test]
    fn config_has_ansi_arrays() {
        let provider = WeztermProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("ansi = {"));
        assert!(config.contains("brights = {"));
    }

    #[test]
    fn launch_args_structure() {
        let provider = WeztermProvider;
        let config_path = Path::new("/tmp/test.lua");
        let args = provider.build_launch_args(config_path, "echo hello");

        assert_eq!(args[0], "wezterm");
        assert_eq!(args[1], "start");
        assert!(args.contains(&"--config-file".to_string()));
        let config_idx = args.iter().position(|a| a == "--config-file").unwrap();
        assert_eq!(args[config_idx + 1], "/tmp/test.lua");
        assert!(args.contains(&"echo hello".to_string()));
    }
}
