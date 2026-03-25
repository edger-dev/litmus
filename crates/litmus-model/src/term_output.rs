use serde::{Deserialize, Serialize};

use crate::provider::ProviderColors;
use crate::Color;

/// A terminal color that can reference theme-dependent or fixed colors.
///
/// Supports the full spectrum from real terminal output:
/// - `Default` — theme foreground or background (context-dependent)
/// - `Ansi(0-15)` — resolved from provider's ANSI palette
/// - `Indexed(16-255)` — fixed xterm-256 color palette
/// - `Rgb(r, g, b)` — 24-bit truecolor, used as-is
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TermColor {
    /// Theme default — foreground or background depending on context.
    Default,
    /// ANSI color (0-15), resolved from the provider's palette.
    Ansi(u8),
    /// 256-color indexed palette (16-255), fixed RGB values.
    Indexed(u8),
    /// 24-bit truecolor, literal RGB.
    Rgb(u8, u8, u8),
}

impl TermColor {
    /// Resolve this color to a concrete RGB value using provider colors.
    ///
    /// `default_color` is the fallback for `Default` — pass the provider's
    /// foreground for fg context, background for bg context.
    pub fn resolve(&self, colors: &ProviderColors, default_color: &Color) -> Color {
        match self {
            TermColor::Default => default_color.clone(),
            TermColor::Ansi(i) => {
                let arr = colors.ansi.as_array();
                arr[(*i as usize).min(15)].clone()
            }
            TermColor::Indexed(i) => indexed_color(*i),
            TermColor::Rgb(r, g, b) => Color::new(*r, *g, *b),
        }
    }

    /// Resolve this color to a concrete RGB value using a `Theme`.
    ///
    /// Same as `resolve` but takes a `Theme` (used by litmus-cli) instead of
    /// `ProviderColors` (used by litmus-web).
    pub fn resolve_with_theme(&self, theme: &crate::Theme, default_color: &Color) -> Color {
        match self {
            TermColor::Default => default_color.clone(),
            TermColor::Ansi(i) => {
                let arr = theme.ansi.as_array();
                arr[(*i as usize).min(15)].clone()
            }
            TermColor::Indexed(i) => indexed_color(*i),
            TermColor::Rgb(r, g, b) => Color::new(*r, *g, *b),
        }
    }
}

/// A span of text with terminal styling from parsed ANSI output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermSpan {
    pub text: String,
    pub fg: TermColor,
    pub bg: TermColor,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub bold: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub dim: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub underline: bool,
}

impl TermSpan {
    /// Create a plain span with default colors and no styling.
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            fg: TermColor::Default,
            bg: TermColor::Default,
            bold: false,
            italic: false,
            dim: false,
            underline: false,
        }
    }
}

/// A single line of terminal output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermLine {
    pub spans: Vec<TermSpan>,
}

impl TermLine {
    pub fn new(spans: Vec<TermSpan>) -> Self {
        Self { spans }
    }

    pub fn empty() -> Self {
        Self { spans: vec![] }
    }
}

/// Parsed terminal output from a fixture, ready for rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermOutput {
    /// Fixture identifier (e.g. "git-diff").
    pub id: String,
    /// Display name (e.g. "Git Diff").
    pub name: String,
    /// Terminal width in columns.
    pub cols: u16,
    /// Terminal height in rows.
    pub rows: u16,
    /// The parsed output lines.
    pub lines: Vec<TermLine>,
}

// -- 256-color palette (xterm) --

/// Look up a color from the standard xterm-256 palette.
///
/// - 0-15: should use Ansi variant instead, but handled here for robustness
/// - 16-231: 6×6×6 color cube
/// - 232-255: 24-step grayscale ramp
fn indexed_color(i: u8) -> Color {
    match i {
        // Standard ANSI colors (0-15) — fallback values if someone puts these in Indexed
        0 => Color::new(0, 0, 0),
        1 => Color::new(128, 0, 0),
        2 => Color::new(0, 128, 0),
        3 => Color::new(128, 128, 0),
        4 => Color::new(0, 0, 128),
        5 => Color::new(128, 0, 128),
        6 => Color::new(0, 128, 128),
        7 => Color::new(192, 192, 192),
        8 => Color::new(128, 128, 128),
        9 => Color::new(255, 0, 0),
        10 => Color::new(0, 255, 0),
        11 => Color::new(255, 255, 0),
        12 => Color::new(0, 0, 255),
        13 => Color::new(255, 0, 255),
        14 => Color::new(0, 255, 255),
        15 => Color::new(255, 255, 255),
        // 6×6×6 color cube: index = 16 + 36*r + 6*g + b (each 0-5)
        16..=231 => {
            let idx = i - 16;
            let b = idx % 6;
            let g = (idx / 6) % 6;
            let r = idx / 36;
            let to_rgb = |c: u8| if c == 0 { 0u8 } else { 55 + 40 * c };
            Color::new(to_rgb(r), to_rgb(g), to_rgb(b))
        }
        // Grayscale ramp: 232-255 → 8, 18, 28, ..., 238
        232..=255 => {
            let v = 8 + 10 * (i - 232);
            Color::new(v, v, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AnsiColors;

    fn test_provider_colors() -> ProviderColors {
        ProviderColors {
            provider: "test".into(),
            source_version: "1.0".into(),
            background: Color::new(0x28, 0x28, 0x28),
            foreground: Color::new(0xeb, 0xdb, 0xb2),
            cursor: Color::new(0xeb, 0xdb, 0xb2),
            selection_background: Color::new(0xeb, 0xdb, 0xb2),
            selection_foreground: Color::new(0x28, 0x28, 0x28),
            ansi: AnsiColors::from_array([
                Color::new(0x28, 0x28, 0x28), // black
                Color::new(0xcc, 0x24, 0x1d), // red
                Color::new(0x98, 0x97, 0x1a), // green
                Color::new(0xd7, 0x99, 0x21), // yellow
                Color::new(0x45, 0x85, 0x88), // blue
                Color::new(0xb1, 0x62, 0x86), // magenta
                Color::new(0x68, 0x9d, 0x6a), // cyan
                Color::new(0xa8, 0x99, 0x84), // white
                Color::new(0x92, 0x83, 0x74), // bright_black
                Color::new(0xfb, 0x49, 0x34), // bright_red
                Color::new(0xb8, 0xbb, 0x26), // bright_green
                Color::new(0xfa, 0xbd, 0x2f), // bright_yellow
                Color::new(0x83, 0xa5, 0x98), // bright_blue
                Color::new(0xd3, 0x86, 0x9b), // bright_magenta
                Color::new(0x8e, 0xc0, 0x7c), // bright_cyan
                Color::new(0xeb, 0xdb, 0xb2), // bright_white
            ]),
        }
    }

    fn test_theme() -> crate::Theme {
        let pc = test_provider_colors();
        crate::Theme {
            name: "Test Theme".into(),
            background: pc.background,
            foreground: pc.foreground,
            cursor: pc.cursor,
            selection_background: pc.selection_background,
            selection_foreground: pc.selection_foreground,
            ansi: pc.ansi,
        }
    }

    // -- TermColor resolve --

    #[test]
    fn resolve_default_uses_fallback() {
        let pc = test_provider_colors();
        let fallback = Color::new(0xff, 0x00, 0x00);
        assert_eq!(TermColor::Default.resolve(&pc, &fallback), fallback);
    }

    #[test]
    fn resolve_ansi_uses_provider_palette() {
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        // Red (index 1)
        assert_eq!(
            TermColor::Ansi(1).resolve(&pc, &dummy),
            Color::new(0xcc, 0x24, 0x1d)
        );
        // Bright white (index 15)
        assert_eq!(
            TermColor::Ansi(15).resolve(&pc, &dummy),
            Color::new(0xeb, 0xdb, 0xb2)
        );
    }

    #[test]
    fn resolve_ansi_clamps_out_of_range() {
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        // Index 99 should clamp to 15
        assert_eq!(
            TermColor::Ansi(99).resolve(&pc, &dummy),
            TermColor::Ansi(15).resolve(&pc, &dummy)
        );
    }

    #[test]
    fn resolve_rgb_is_literal() {
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        assert_eq!(
            TermColor::Rgb(0xde, 0xad, 0xbe).resolve(&pc, &dummy),
            Color::new(0xde, 0xad, 0xbe)
        );
    }

    // -- TermColor resolve_with_theme --

    #[test]
    fn resolve_with_theme_default_fg() {
        let theme = test_theme();
        assert_eq!(
            TermColor::Default.resolve_with_theme(&theme, &theme.foreground),
            theme.foreground
        );
    }

    #[test]
    fn resolve_with_theme_default_bg() {
        let theme = test_theme();
        assert_eq!(
            TermColor::Default.resolve_with_theme(&theme, &theme.background),
            theme.background
        );
    }

    #[test]
    fn resolve_with_theme_ansi_matches_provider() {
        let theme = test_theme();
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        // All 16 ANSI colors should match between resolve and resolve_with_theme
        for i in 0..16u8 {
            assert_eq!(
                TermColor::Ansi(i).resolve_with_theme(&theme, &dummy),
                TermColor::Ansi(i).resolve(&pc, &dummy),
                "ANSI color {i} mismatch"
            );
        }
    }

    #[test]
    fn resolve_with_theme_indexed_matches_provider() {
        let theme = test_theme();
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        for i in [16u8, 100, 200, 232, 255] {
            assert_eq!(
                TermColor::Indexed(i).resolve_with_theme(&theme, &dummy),
                TermColor::Indexed(i).resolve(&pc, &dummy),
                "Indexed color {i} mismatch"
            );
        }
    }

    #[test]
    fn resolve_with_theme_rgb_literal() {
        let theme = test_theme();
        let dummy = Color::new(0, 0, 0);
        assert_eq!(
            TermColor::Rgb(0xab, 0xcd, 0xef).resolve_with_theme(&theme, &dummy),
            Color::new(0xab, 0xcd, 0xef)
        );
    }

    // -- Indexed color table --

    #[test]
    fn indexed_color_cube_corners() {
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        // Color 16 = cube(0,0,0) = black
        assert_eq!(
            TermColor::Indexed(16).resolve(&pc, &dummy),
            Color::new(0, 0, 0)
        );
        // Color 231 = cube(5,5,5) = white-ish
        assert_eq!(
            TermColor::Indexed(231).resolve(&pc, &dummy),
            Color::new(255, 255, 255)
        );
        // Color 196 = cube(5,0,0) = pure red
        // 196 = 16 + 36*5 + 6*0 + 0
        assert_eq!(
            TermColor::Indexed(196).resolve(&pc, &dummy),
            Color::new(255, 0, 0)
        );
        // Color 21 = cube(0,0,5) = pure blue
        // 21 = 16 + 36*0 + 6*0 + 5
        assert_eq!(
            TermColor::Indexed(21).resolve(&pc, &dummy),
            Color::new(0, 0, 255)
        );
    }

    #[test]
    fn indexed_grayscale() {
        let pc = test_provider_colors();
        let dummy = Color::new(0, 0, 0);
        // Color 232 = darkest gray = rgb(8,8,8)
        assert_eq!(
            TermColor::Indexed(232).resolve(&pc, &dummy),
            Color::new(8, 8, 8)
        );
        // Color 255 = lightest gray = rgb(238,238,238)
        assert_eq!(
            TermColor::Indexed(255).resolve(&pc, &dummy),
            Color::new(238, 238, 238)
        );
    }

    // -- Serde round-trips --

    #[test]
    fn term_color_serde_round_trip() {
        let colors = vec![
            TermColor::Default,
            TermColor::Ansi(5),
            TermColor::Indexed(200),
            TermColor::Rgb(0xab, 0xcd, 0xef),
        ];
        for color in &colors {
            let json = serde_json::to_string(color).unwrap();
            let parsed: TermColor = serde_json::from_str(&json).unwrap();
            assert_eq!(&parsed, color, "round-trip failed for {json}");
        }
    }

    #[test]
    fn term_span_serde_skips_false_attrs() {
        let span = TermSpan::plain("hello");
        let json = serde_json::to_string(&span).unwrap();
        // bold/italic/dim/underline should be omitted when false
        assert!(!json.contains("bold"), "should skip false bold");
        assert!(!json.contains("italic"), "should skip false italic");
    }

    #[test]
    fn term_span_serde_includes_true_attrs() {
        let span = TermSpan {
            bold: true,
            italic: true,
            ..TermSpan::plain("hello")
        };
        let json = serde_json::to_string(&span).unwrap();
        assert!(json.contains("\"bold\":true"));
        assert!(json.contains("\"italic\":true"));
        // Round-trip
        let parsed: TermSpan = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, span);
    }

    #[test]
    fn term_output_serde_round_trip() {
        let output = TermOutput {
            id: "git-diff".into(),
            name: "Git Diff".into(),
            cols: 80,
            rows: 24,
            lines: vec![
                TermLine::new(vec![
                    TermSpan::plain("$ "),
                    TermSpan {
                        text: "git diff".into(),
                        fg: TermColor::Ansi(2),
                        bg: TermColor::Default,
                        bold: true,
                        italic: false,
                        dim: false,
                        underline: false,
                    },
                ]),
                TermLine::new(vec![TermSpan {
                    text: "-old line".into(),
                    fg: TermColor::Ansi(1),
                    bg: TermColor::Default,
                    bold: false,
                    italic: false,
                    dim: false,
                    underline: false,
                }]),
                TermLine::new(vec![TermSpan {
                    text: "+new line".into(),
                    fg: TermColor::Ansi(2),
                    bg: TermColor::Default,
                    bold: false,
                    italic: false,
                    dim: false,
                    underline: false,
                }]),
                TermLine::empty(),
            ],
        };

        let json = serde_json::to_string_pretty(&output).unwrap();
        let parsed: TermOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, output);
        assert_eq!(parsed.cols, 80);
        assert_eq!(parsed.rows, 24);
        assert_eq!(parsed.lines.len(), 4);
        assert!(parsed.lines[3].spans.is_empty());
    }

    #[test]
    fn term_output_deserialization_defaults_missing_attrs() {
        // JSON with no bold/italic/dim/underline should default to false
        let json = r#"{
            "id": "test",
            "name": "Test",
            "cols": 80,
            "rows": 24,
            "lines": [{
                "spans": [{
                    "text": "hello",
                    "fg": {"type": "Default"},
                    "bg": {"type": "Default"}
                }]
            }]
        }"#;
        let output: TermOutput = serde_json::from_str(json).unwrap();
        let span = &output.lines[0].spans[0];
        assert!(!span.bold);
        assert!(!span.italic);
        assert!(!span.dim);
        assert!(!span.underline);
    }
}
