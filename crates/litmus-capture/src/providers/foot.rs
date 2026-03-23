use litmus_model::Theme;
use std::path::Path;

use super::{ProviderCapture, TermGeometry};

pub struct FootProvider;

impl ProviderCapture for FootProvider {
    fn slug(&self) -> &str {
        "foot"
    }

    fn name(&self) -> &str {
        "Foot"
    }

    fn config_extension(&self) -> &str {
        "ini"
    }

    fn generate_config(&self, theme: &Theme, geometry: &TermGeometry) -> String {
        let bg = &theme.background;
        let fg = &theme.foreground;
        let cur = &theme.cursor;
        let sel_bg = &theme.selection_background;
        let sel_fg = &theme.selection_foreground;
        let ansi = theme.ansi.as_array();

        // foot config uses hex colors without '#' prefix
        let hex = |c: &litmus_model::Color| format!("{:02x}{:02x}{:02x}", c.r, c.g, c.b);

        let mut config = String::new();

        config.push_str("[main]\n");
        config.push_str(&format!("font={}:size={:.0}\n", geometry.font_family, geometry.font_size));
        config.push_str(&format!("initial-window-size-pixels={}x{}\n",
            geometry.cols * 10, geometry.rows * 22)); // approx pixel size
        config.push_str("pad=4x4\n");

        config.push_str("\n[cursor]\n");
        config.push_str("blink=no\n");

        // foot 1.17+: colors live in [colors-dark] section
        config.push_str("\n[colors-dark]\n");
        config.push_str(&format!("background={}\n", hex(bg)));
        config.push_str(&format!("foreground={}\n", hex(fg)));
        // cursor = <cursor-bg> <cursor-text> (two space-separated RRGGBB values)
        config.push_str(&format!("cursor={} {}\n", hex(cur), hex(bg)));
        config.push_str(&format!("selection-background={}\n", hex(sel_bg)));
        config.push_str(&format!("selection-foreground={}\n", hex(sel_fg)));
        config.push_str("\n");
        // Regular ANSI colors (0-7)
        for i in 0..8 {
            config.push_str(&format!("regular{}={}\n", i, hex(&ansi[i])));
        }
        config.push_str("\n");
        // Bright ANSI colors (8-15)
        for i in 0..8 {
            config.push_str(&format!("bright{}={}\n", i, hex(&ansi[8 + i])));
        }

        config
    }

    fn build_launch_args(&self, config_path: &Path, command: &str) -> Vec<String> {
        vec![
            "foot".to_string(),
            "--config".to_string(),
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
    fn config_contains_colors() {
        let provider = FootProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("[colors-dark]"));
        // background = 1e1e2e
        assert!(config.contains("background=1e1e2e"));
        // foreground = cdd6f4
        assert!(config.contains("foreground=cdd6f4"));
        // regular0 = 45475a
        assert!(config.contains("regular0=45475a"));
    }

    #[test]
    fn config_contains_font() {
        let provider = FootProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        assert!(config.contains("FiraCode"));
    }

    #[test]
    fn config_has_all_ansi_colors() {
        let provider = FootProvider;
        let theme = sample_theme();
        let config = provider.generate_config(&theme, &TermGeometry::default());

        for i in 0..8 {
            assert!(config.contains(&format!("regular{}=", i)));
            assert!(config.contains(&format!("bright{}=", i)));
        }
    }

    #[test]
    fn launch_args_structure() {
        let provider = FootProvider;
        let config_path = Path::new("/tmp/test.ini");
        let args = provider.build_launch_args(config_path, "echo hello");

        assert_eq!(args[0], "foot");
        assert!(args.contains(&"--config".to_string()));
        let config_idx = args.iter().position(|a| a == "--config").unwrap();
        assert_eq!(args[config_idx + 1], "/tmp/test.ini");
        assert!(args.contains(&"echo hello".to_string()));
    }
}
