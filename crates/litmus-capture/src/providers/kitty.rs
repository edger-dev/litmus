use litmus_model::{export::to_kitty_conf, Theme};
use std::path::Path;

use super::{ProviderCapture, TermGeometry};

pub struct KittyProvider;

impl ProviderCapture for KittyProvider {
    fn slug(&self) -> &str {
        "kitty"
    }

    fn name(&self) -> &str {
        "Kitty"
    }

    fn config_extension(&self) -> &str {
        "conf"
    }

    fn generate_config(&self, theme: &Theme, geometry: &TermGeometry) -> String {
        // Start with the standard kitty theme export (colors only)
        let mut config = to_kitty_conf(theme);

        // Append capture-specific settings
        config.push('\n');
        config.push_str(&format!("font_family {}\n", geometry.font_family));
        config.push_str(&format!("font_size {:.1}\n", geometry.font_size));
        config.push_str(&format!("initial_window_columns {}\n", geometry.cols));
        config.push_str(&format!("initial_window_rows {}\n", geometry.rows));

        // Disable features that could interfere with screenshot accuracy
        config.push_str("remember_window_size no\n");
        config.push_str("confirm_os_window_close 0\n");
        config.push_str("hide_window_decorations yes\n");
        config.push_str("enable_audio_bell no\n");
        config.push_str("update_check_interval 0\n");
        // Disable tab bar (we're capturing a single window)
        config.push_str("tab_bar_style hidden\n");
        // Ensure cursor doesn't blink (for consistent screenshots)
        config.push_str("cursor_blink_interval 0\n");

        config
    }

    fn build_launch_args(&self, config_path: &Path, command: &str) -> Vec<String> {
        vec![
            "kitty".to_string(),
            "--config".to_string(),
            config_path.to_string_lossy().into_owned(),
            // Run the command; the capture wrapper manages the lifecycle
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
                c(69, 71, 90),   // black
                c(243, 139, 168), // red
                c(166, 227, 161), // green
                c(249, 226, 175), // yellow
                c(137, 180, 250), // blue
                c(245, 194, 231), // magenta
                c(148, 226, 213), // cyan
                c(186, 194, 222), // white
                c(88, 91, 112),  // bright black
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
        let provider = KittyProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("background #1E1E2E"));
        assert!(config.contains("foreground #CDD6F4"));
        assert!(config.contains("color0 #45475A"));
    }

    #[test]
    fn config_contains_geometry() {
        let provider = KittyProvider;
        let theme = sample_theme();
        let geometry = TermGeometry {
            cols: 80,
            rows: 24,
            font_size: 12.0,
            font_family: "FiraCode".to_string(),
        };
        let config = provider.generate_config(&theme, &geometry);

        assert!(config.contains("font_family FiraCode"));
        assert!(config.contains("font_size 12.0"));
        assert!(config.contains("initial_window_columns 80"));
        assert!(config.contains("initial_window_rows 24"));
    }

    #[test]
    fn config_has_screenshot_settings() {
        let provider = KittyProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("hide_window_decorations yes"));
        assert!(config.contains("confirm_os_window_close 0"));
        assert!(config.contains("cursor_blink_interval 0"));
        assert!(config.contains("tab_bar_style hidden"));
    }

    #[test]
    fn launch_args_structure() {
        let provider = KittyProvider;
        let config_path = Path::new("/tmp/test.conf");
        let args = provider.build_launch_args(config_path, "echo hello");

        assert_eq!(args[0], "kitty");
        assert!(args.contains(&"--config".to_string()));
        let config_idx = args.iter().position(|a| a == "--config").unwrap();
        assert_eq!(args[config_idx + 1], "/tmp/test.conf");
        assert!(args.contains(&"echo hello".to_string()));
    }
}
