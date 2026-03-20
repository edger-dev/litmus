use crate::Theme;

/// Export theme as kitty.conf color settings.
pub fn to_kitty_conf(theme: &Theme) -> String {
    let ansi = theme.ansi.as_array();
    let mut out = String::new();
    out.push_str(&format!("# {}\n", theme.name));
    out.push_str(&format!("background {}\n", theme.background.to_hex()));
    out.push_str(&format!("foreground {}\n", theme.foreground.to_hex()));
    out.push_str(&format!("cursor {}\n", theme.cursor.to_hex()));
    out.push_str(&format!(
        "selection_background {}\n",
        theme.selection_background.to_hex()
    ));
    out.push_str(&format!(
        "selection_foreground {}\n",
        theme.selection_foreground.to_hex()
    ));
    out.push('\n');
    for (i, color) in ansi.iter().enumerate() {
        out.push_str(&format!("color{i} {}\n", color.to_hex()));
    }
    out
}

/// Export theme as TOML (litmus canonical format).
pub fn to_toml(theme: &Theme) -> String {
    let ansi = theme.ansi.as_array();
    let ansi_names = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
        "bright_black",
        "bright_red",
        "bright_green",
        "bright_yellow",
        "bright_blue",
        "bright_magenta",
        "bright_cyan",
        "bright_white",
    ];
    let mut out = String::new();
    out.push_str(&format!("name = \"{}\"\n", theme.name));
    out.push_str(&format!(
        "background = \"{}\"\n",
        theme.background.to_hex()
    ));
    out.push_str(&format!(
        "foreground = \"{}\"\n",
        theme.foreground.to_hex()
    ));
    out.push_str(&format!("cursor = \"{}\"\n", theme.cursor.to_hex()));
    out.push_str(&format!(
        "selection_background = \"{}\"\n",
        theme.selection_background.to_hex()
    ));
    out.push_str(&format!(
        "selection_foreground = \"{}\"\n",
        theme.selection_foreground.to_hex()
    ));
    out.push_str("\n[ansi]\n");
    for (i, name) in ansi_names.iter().enumerate() {
        out.push_str(&format!("{name} = \"{}\"\n", ansi[i].to_hex()));
    }
    out
}

/// Export theme as a Nix attrset for programs.kitty.settings.
pub fn to_nix(theme: &Theme) -> String {
    let ansi = theme.ansi.as_array();
    let hex = |c: &crate::Color| c.to_hex();
    let mut out = String::new();
    out.push_str(&format!("# {}\n", theme.name));
    out.push_str("{\n");
    out.push_str(&format!("  background = \"{}\";\n", hex(&theme.background)));
    out.push_str(&format!("  foreground = \"{}\";\n", hex(&theme.foreground)));
    out.push_str(&format!("  cursor = \"{}\";\n", hex(&theme.cursor)));
    out.push_str(&format!(
        "  selection_background = \"{}\";\n",
        hex(&theme.selection_background)
    ));
    out.push_str(&format!(
        "  selection_foreground = \"{}\";\n",
        hex(&theme.selection_foreground)
    ));
    for (i, color) in ansi.iter().enumerate() {
        out.push_str(&format!("  color{i} = \"{}\";\n", hex(color)));
    }
    out.push_str("}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnsiColors, Color};

    fn test_theme() -> Theme {
        let c = |hex: &str| Color::from_hex(hex).unwrap();
        Theme {
            name: "Test".to_string(),
            background: c("#1a1b26"),
            foreground: c("#c0caf5"),
            cursor: c("#c0caf5"),
            selection_background: c("#33467c"),
            selection_foreground: c("#c0caf5"),
            ansi: AnsiColors::from_array(std::array::from_fn(|i| {
                Color::new((i * 16) as u8, (i * 16) as u8, (i * 16) as u8)
            })),
        }
    }

    #[test]
    fn kitty_conf_contains_colors() {
        let conf = to_kitty_conf(&test_theme());
        assert!(conf.contains("background #1A1B26"));
        assert!(conf.contains("foreground #C0CAF5"));
        assert!(conf.contains("color0 #000000"));
        assert!(conf.contains("color15 #F0F0F0"));
    }

    #[test]
    fn toml_format_is_valid() {
        let toml = to_toml(&test_theme());
        assert!(toml.contains("name = \"Test\""));
        assert!(toml.contains("[ansi]"));
        assert!(toml.contains("black = \"#000000\""));
    }

    #[test]
    fn nix_format_has_braces() {
        let nix = to_nix(&test_theme());
        assert!(nix.starts_with("# Test\n{"));
        assert!(nix.contains("background = \"#1A1B26\";"));
        assert!(nix.ends_with("}\n"));
    }
}
