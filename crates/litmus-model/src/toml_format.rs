use serde::Deserialize;

use crate::{AnsiColors, Theme, defaults, error::ThemeError, parse_hex_color};

#[derive(Debug, Deserialize)]
struct TomlThemeRaw {
    name: String,
    colors: TomlColorsRaw,
}

#[derive(Debug, Deserialize)]
struct TomlColorsRaw {
    background: String,
    foreground: String,
    cursor: Option<String>,
    selection_background: Option<String>,
    selection_foreground: Option<String>,
    ansi: TomlAnsiRaw,
}

#[derive(Debug, Deserialize)]
struct TomlAnsiRaw {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
    bright_black: String,
    bright_red: String,
    bright_green: String,
    bright_yellow: String,
    bright_blue: String,
    bright_magenta: String,
    bright_cyan: String,
    bright_white: String,
}

pub fn parse_toml_theme(input: &str) -> Result<Theme, ThemeError> {
    let raw: TomlThemeRaw = toml::from_str(input)?;

    let background = parse_hex_color("background", &raw.colors.background)?;
    let foreground = parse_hex_color("foreground", &raw.colors.foreground)?;

    let cursor = raw.colors.cursor.as_deref()
        .map(|v| parse_hex_color("cursor", v))
        .transpose()?
        .unwrap_or_else(|| defaults::default_cursor(&foreground));

    let selection_background = raw.colors.selection_background.as_deref()
        .map(|v| parse_hex_color("selection_background", v))
        .transpose()?
        .unwrap_or_else(|| defaults::default_selection_bg(&background));

    let selection_foreground = raw.colors.selection_foreground.as_deref()
        .map(|v| parse_hex_color("selection_foreground", v))
        .transpose()?
        .unwrap_or_else(|| defaults::default_selection_fg(&foreground));

    let a = &raw.colors.ansi;
    let ansi = AnsiColors {
        black: parse_hex_color("black", &a.black)?,
        red: parse_hex_color("red", &a.red)?,
        green: parse_hex_color("green", &a.green)?,
        yellow: parse_hex_color("yellow", &a.yellow)?,
        blue: parse_hex_color("blue", &a.blue)?,
        magenta: parse_hex_color("magenta", &a.magenta)?,
        cyan: parse_hex_color("cyan", &a.cyan)?,
        white: parse_hex_color("white", &a.white)?,
        bright_black: parse_hex_color("bright_black", &a.bright_black)?,
        bright_red: parse_hex_color("bright_red", &a.bright_red)?,
        bright_green: parse_hex_color("bright_green", &a.bright_green)?,
        bright_yellow: parse_hex_color("bright_yellow", &a.bright_yellow)?,
        bright_blue: parse_hex_color("bright_blue", &a.bright_blue)?,
        bright_magenta: parse_hex_color("bright_magenta", &a.bright_magenta)?,
        bright_cyan: parse_hex_color("bright_cyan", &a.bright_cyan)?,
        bright_white: parse_hex_color("bright_white", &a.bright_white)?,
    };

    Ok(Theme {
        name: raw.name,
        background,
        foreground,
        cursor,
        selection_background,
        selection_foreground,
        ansi,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    const FULL_TOML: &str = r##"
name = "Test Theme"

[colors]
background = "#1a1b26"
foreground = "#a9b1d6"
cursor = "#c0caf5"
selection_background = "#283d5c"
selection_foreground = "#ffffff"

[colors.ansi]
black = "#151622"
red = "#f7768e"
green = "#9ece6a"
yellow = "#e0af68"
blue = "#7aa2f7"
magenta = "#bb9af7"
cyan = "#7dcfff"
white = "#a9b1d6"
bright_black = "#414468"
bright_red = "#f7768e"
bright_green = "#9ece6a"
bright_yellow = "#e0af68"
bright_blue = "#7aa2f7"
bright_magenta = "#bb9af7"
bright_cyan = "#7dcfff"
bright_white = "#c0caf5"
"##;

    #[test]
    fn parse_full_toml() {
        let t = parse_toml_theme(FULL_TOML).unwrap();
        assert_eq!(t.name, "Test Theme");
        assert_eq!(t.background, Color::new(0x1a, 0x1b, 0x26));
        assert_eq!(t.foreground, Color::new(0xa9, 0xb1, 0xd6));
        assert_eq!(t.cursor, Color::new(0xc0, 0xca, 0xf5));
        assert_eq!(t.selection_background, Color::new(0x28, 0x3d, 0x5c));
        assert_eq!(t.ansi.black, Color::new(0x15, 0x16, 0x22));
        assert_eq!(t.ansi.bright_white, Color::new(0xc0, 0xca, 0xf5));
    }

    #[test]
    fn parse_omitted_optionals() {
        let minimal = FULL_TOML
            .lines()
            .filter(|l| {
                !l.trim_start().starts_with("cursor")
                    && !l.trim_start().starts_with("selection")
            })
            .collect::<Vec<_>>()
            .join("\n");
        let t = parse_toml_theme(&minimal).unwrap();
        // defaults applied
        assert_eq!(t.cursor, t.foreground);
        assert_eq!(
            t.selection_background,
            Color::new(
                t.background.r.saturating_add(0x28),
                t.background.g.saturating_add(0x28),
                t.background.b.saturating_add(0x28),
            )
        );
    }

    #[test]
    fn invalid_toml_returns_error() {
        let bad = "this is not toml :::";
        assert!(matches!(parse_toml_theme(bad), Err(ThemeError::Toml(_))));
    }
}
