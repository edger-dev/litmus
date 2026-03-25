use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use litmus_model::Theme;
use litmus_model::term_output::{TermColor, TermLine, TermOutput};
use super::util::to_ratatui_color;

/// Embedded fixture data (JSON) for the mockups view.
static FIXTURE_DATA: &[(&str, &str)] = &[
    ("git-diff", include_str!("../../../../fixtures/git-diff/output.json")),
    ("git-log", include_str!("../../../../fixtures/git-log/output.json")),
    ("ls-color", include_str!("../../../../fixtures/ls-color/output.json")),
    ("cargo-build", include_str!("../../../../fixtures/cargo-build/output.json")),
    ("shell-prompt", include_str!("../../../../fixtures/shell-prompt/output.json")),
];

fn load_fixtures() -> Vec<TermOutput> {
    FIXTURE_DATA
        .iter()
        .filter_map(|(id, json)| {
            serde_json::from_str::<TermOutput>(json)
                .map_err(|e| eprintln!("Warning: failed to parse fixture {id}: {e}"))
                .ok()
        })
        .collect()
}

/// Resolve a `TermColor` to a ratatui `Color` using the theme palette.
fn resolve_color(tc: &TermColor, theme: &Theme, default: &litmus_model::Color) -> ratatui::style::Color {
    to_ratatui_color(&tc.resolve_with_theme(theme, default))
}

/// Convert a `TermLine` to a ratatui `Line` using the given theme.
fn term_line_to_ratatui(line: &TermLine, theme: &Theme) -> Line<'static> {
    let spans: Vec<Span<'static>> = line
        .spans
        .iter()
        .map(|span| {
            let fg = resolve_color(&span.fg, theme, &theme.foreground);
            let bg = resolve_color(&span.bg, theme, &theme.background);
            let mut style = Style::default().fg(fg).bg(bg);
            if span.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if span.italic {
                style = style.add_modifier(Modifier::ITALIC);
            }
            if span.dim {
                style = style.add_modifier(Modifier::DIM);
            }
            if span.underline {
                style = style.add_modifier(Modifier::UNDERLINED);
            }
            Span::styled(span.text.clone(), style)
        })
        .collect();
    Line::from(spans)
}

pub struct MockupsWidget<'a> {
    pub theme: &'a Theme,
    pub fixture_index: usize,
}

impl<'a> Widget for MockupsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let t = self.theme;
        let fg = to_ratatui_color(&t.foreground);
        let bg = to_ratatui_color(&t.background);
        let base = Style::default().fg(fg).bg(bg);

        // Fill background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_style(base).set_char(' ');
            }
        }

        let fixtures = load_fixtures();
        if fixtures.is_empty() {
            let line = Line::from(Span::styled("(no fixture data)", base));
            let rect = Rect { x: area.left(), y: area.top(), width: area.width, height: 1 };
            line.render(rect, buf);
            return;
        }

        let fixture = &fixtures[self.fixture_index % fixtures.len()];

        // Render fixture name header
        let header = Line::from(vec![
            Span::styled(
                format!(" {} ", fixture.name),
                Style::default()
                    .fg(to_ratatui_color(&t.background))
                    .bg(to_ratatui_color(&t.foreground))
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        if area.height > 0 {
            let rect = Rect { x: area.left(), y: area.top(), width: area.width, height: 1 };
            header.render(rect, buf);
        }

        // Render fixture lines
        for (i, term_line) in fixture.lines.iter().enumerate() {
            let y = area.top() + 1 + i as u16;
            if y >= area.bottom() {
                break;
            }
            let line = term_line_to_ratatui(term_line, t);
            let rect = Rect { x: area.left(), y, width: area.width, height: 1 };
            line.render(rect, buf);
        }
    }
}

/// Return the number of embedded fixtures.
pub fn fixture_count() -> usize {
    FIXTURE_DATA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use litmus_model::term_output::TermSpan;
    use litmus_model::{AnsiColors, Color};

    fn test_theme() -> Theme {
        Theme {
            name: "Test".into(),
            background: Color::new(0x1a, 0x1b, 0x26),
            foreground: Color::new(0xc0, 0xca, 0xf5),
            cursor: Color::new(0xc0, 0xca, 0xf5),
            selection_background: Color::new(0x28, 0x3b, 0x8c),
            selection_foreground: Color::new(0xc0, 0xca, 0xf5),
            ansi: AnsiColors::from_array([
                Color::new(0x15, 0x16, 0x1e), // 0 black
                Color::new(0xf7, 0x76, 0x8e), // 1 red
                Color::new(0x9e, 0xce, 0x6a), // 2 green
                Color::new(0xe0, 0xaf, 0x68), // 3 yellow
                Color::new(0x7a, 0xa2, 0xf7), // 4 blue
                Color::new(0xbb, 0x9a, 0xf7), // 5 magenta
                Color::new(0x7d, 0xcf, 0xff), // 6 cyan
                Color::new(0xa9, 0xb1, 0xd6), // 7 white
                Color::new(0x41, 0x48, 0x68), // 8 bright black
                Color::new(0xf7, 0x76, 0x8e), // 9 bright red
                Color::new(0x9e, 0xce, 0x6a), // 10 bright green
                Color::new(0xe0, 0xaf, 0x68), // 11 bright yellow
                Color::new(0x7a, 0xa2, 0xf7), // 12 bright blue
                Color::new(0xbb, 0x9a, 0xf7), // 13 bright magenta
                Color::new(0x7d, 0xcf, 0xff), // 14 bright cyan
                Color::new(0xc0, 0xca, 0xf5), // 15 bright white
            ]),
        }
    }

    #[test]
    fn resolve_default_fg_uses_theme_foreground() {
        let theme = test_theme();
        let result = resolve_color(&TermColor::Default, &theme, &theme.foreground);
        assert_eq!(result, ratatui::style::Color::Rgb(0xc0, 0xca, 0xf5));
    }

    #[test]
    fn resolve_default_bg_uses_theme_background() {
        let theme = test_theme();
        let result = resolve_color(&TermColor::Default, &theme, &theme.background);
        assert_eq!(result, ratatui::style::Color::Rgb(0x1a, 0x1b, 0x26));
    }

    #[test]
    fn resolve_ansi_uses_theme_palette() {
        let theme = test_theme();
        // Red (index 1) should use theme's ANSI red
        let result = resolve_color(&TermColor::Ansi(1), &theme, &theme.foreground);
        assert_eq!(result, ratatui::style::Color::Rgb(0xf7, 0x76, 0x8e));
    }

    #[test]
    fn resolve_rgb_is_literal() {
        let theme = test_theme();
        let result = resolve_color(&TermColor::Rgb(0xde, 0xad, 0xbe), &theme, &theme.foreground);
        assert_eq!(result, ratatui::style::Color::Rgb(0xde, 0xad, 0xbe));
    }

    #[test]
    fn term_line_to_ratatui_preserves_styles() {
        let theme = test_theme();
        let line = TermLine::new(vec![
            TermSpan::plain("normal "),
            TermSpan {
                text: "bold red".into(),
                fg: TermColor::Ansi(1),
                bg: TermColor::Default,
                bold: true,
                italic: false,
                dim: false,
                underline: false,
            },
        ]);
        let ratatui_line = term_line_to_ratatui(&line, &theme);
        assert_eq!(ratatui_line.spans.len(), 2);
        assert_eq!(ratatui_line.spans[0].content, "normal ");
        assert_eq!(ratatui_line.spans[1].content, "bold red");
        assert!(ratatui_line.spans[1].style.add_modifier == Modifier::BOLD);
    }

    #[test]
    fn load_embedded_fixtures_succeeds() {
        let fixtures = load_fixtures();
        assert!(fixtures.len() >= 3, "expected at least 3 embedded fixtures, got {}", fixtures.len());
        for f in &fixtures {
            assert!(!f.id.is_empty());
            assert!(!f.lines.is_empty(), "fixture {} has no lines", f.id);
        }
    }

    #[test]
    fn fixture_count_matches() {
        assert_eq!(fixture_count(), FIXTURE_DATA.len());
        assert_eq!(fixture_count(), load_fixtures().len());
    }
}
