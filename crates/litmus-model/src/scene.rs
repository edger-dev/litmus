use serde::{Deserialize, Serialize};

use crate::{Color, Theme};

/// A semantic color reference that resolves against a theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeColor {
    /// Theme foreground color.
    Foreground,
    /// Theme background color.
    Background,
    /// Theme cursor color.
    Cursor,
    /// Theme selection background color.
    SelectionBackground,
    /// Theme selection foreground color.
    SelectionForeground,
    /// ANSI color by index (0-15).
    Ansi(u8),
}

impl ThemeColor {
    /// Return a short stable identifier for this color reference.
    pub fn slug(&self) -> String {
        match self {
            ThemeColor::Foreground => "fg".into(),
            ThemeColor::Background => "bg".into(),
            ThemeColor::Cursor => "cursor".into(),
            ThemeColor::SelectionBackground => "sel-bg".into(),
            ThemeColor::SelectionForeground => "sel-fg".into(),
            ThemeColor::Ansi(i) => format!("ansi{i}"),
        }
    }

    /// Resolve this semantic color reference to a concrete color from the theme.
    pub fn resolve<'a>(&self, theme: &'a Theme) -> &'a Color {
        match self {
            ThemeColor::Foreground => &theme.foreground,
            ThemeColor::Background => &theme.background,
            ThemeColor::Cursor => &theme.cursor,
            ThemeColor::SelectionBackground => &theme.selection_background,
            ThemeColor::SelectionForeground => &theme.selection_foreground,
            ThemeColor::Ansi(i) => {
                let arr = theme.ansi.as_array();
                arr[(*i as usize).min(15)]
            }
        }
    }
}

/// Text modifiers (bold, italic, etc.).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub bold: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub italic: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub underline: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub dim: bool,
}

/// A span of text with semantic styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyledSpan {
    pub text: String,
    /// Foreground color (defaults to theme foreground if None).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fg: Option<ThemeColor>,
    /// Background color (defaults to theme background if None).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<ThemeColor>,
    /// Text modifiers.
    #[serde(default, skip_serializing_if = "is_default_style")]
    pub style: TextStyle,
}

fn is_default_style(s: &TextStyle) -> bool {
    *s == TextStyle::default()
}

impl StyledSpan {
    /// Create a plain span with default foreground/background.
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            fg: None,
            bg: None,
            style: TextStyle::default(),
        }
    }

    /// Create a span with a specific foreground color.
    pub fn colored(text: impl Into<String>, fg: ThemeColor) -> Self {
        Self {
            text: text.into(),
            fg: Some(fg),
            bg: None,
            style: TextStyle::default(),
        }
    }

    /// Add bold modifier.
    pub fn bold(mut self) -> Self {
        self.style.bold = true;
        self
    }

    /// Add italic modifier.
    pub fn italic(mut self) -> Self {
        self.style.italic = true;
        self
    }

    /// Add dim modifier.
    pub fn dim(mut self) -> Self {
        self.style.dim = true;
        self
    }

    /// Set background color.
    pub fn on(mut self, bg: ThemeColor) -> Self {
        self.bg = Some(bg);
        self
    }
}

/// A single line in a scene, composed of styled spans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneLine {
    pub spans: Vec<StyledSpan>,
}

impl SceneLine {
    pub fn new(spans: Vec<StyledSpan>) -> Self {
        Self { spans }
    }

    pub fn empty() -> Self {
        Self { spans: vec![] }
    }
}

/// A terminal scene: a sequence of styled lines with metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    /// Machine-readable identifier (e.g. "git-diff").
    pub id: String,
    /// Human-readable name (e.g. "Git Diff").
    pub name: String,
    /// Brief description of what this scene demonstrates.
    pub description: String,
    /// The lines of styled content.
    pub lines: Vec<SceneLine>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_color_resolve() {
        let theme = crate::Theme {
            name: "test".into(),
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            cursor: Color::new(255, 0, 0),
            selection_background: Color::new(50, 50, 50),
            selection_foreground: Color::new(200, 200, 200),
            ansi: crate::AnsiColors::from_array(std::array::from_fn(|i| {
                Color::new(i as u8 * 16, i as u8 * 16, i as u8 * 16)
            })),
        };

        assert_eq!(*ThemeColor::Foreground.resolve(&theme), Color::new(255, 255, 255));
        assert_eq!(*ThemeColor::Background.resolve(&theme), Color::new(0, 0, 0));
        assert_eq!(*ThemeColor::Ansi(2).resolve(&theme), Color::new(32, 32, 32));
        // Out-of-range clamps to 15
        assert_eq!(*ThemeColor::Ansi(99).resolve(&theme), Color::new(240, 240, 240));
    }

    #[test]
    fn styled_span_builder() {
        let span = StyledSpan::colored("hello", ThemeColor::Ansi(1)).bold();
        assert_eq!(span.fg, Some(ThemeColor::Ansi(1)));
        assert!(span.style.bold);
        assert!(!span.style.italic);
    }

    #[test]
    fn scene_serialization_round_trip() {
        let scene = Scene {
            id: "test".into(),
            name: "Test Scene".into(),
            description: "A test".into(),
            lines: vec![
                SceneLine::new(vec![
                    StyledSpan::plain("hello "),
                    StyledSpan::colored("world", ThemeColor::Ansi(1)).bold(),
                ]),
                SceneLine::empty(),
            ],
        };

        let json = serde_json::to_string(&scene).unwrap();
        let deserialized: Scene = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "test");
        assert_eq!(deserialized.lines.len(), 2);
        assert_eq!(deserialized.lines[0].spans[1].fg, Some(ThemeColor::Ansi(1)));
    }
}
