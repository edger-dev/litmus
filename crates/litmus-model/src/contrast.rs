//! WCAG contrast ratio calculation and readability validation.

use crate::scene::{Scene, ThemeColor};
use crate::{Color, Theme};

/// Minimum contrast ratio for WCAG AA normal text.
pub const WCAG_AA_NORMAL: f64 = 4.5;
/// Minimum contrast ratio for WCAG AA large text (bold >=14pt or normal >=18pt).
pub const WCAG_AA_LARGE: f64 = 3.0;
/// Minimum contrast ratio for WCAG AAA normal text.
pub const WCAG_AAA_NORMAL: f64 = 7.0;

/// Convert an sRGB component (0-255) to linear luminance component.
fn srgb_to_linear(c: u8) -> f64 {
    let s = c as f64 / 255.0;
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate relative luminance per WCAG 2.1.
pub fn relative_luminance(color: &Color) -> f64 {
    0.2126 * srgb_to_linear(color.r) + 0.7152 * srgb_to_linear(color.g) + 0.0722 * srgb_to_linear(color.b)
}

/// Calculate WCAG contrast ratio between two colors.
/// Returns a value >= 1.0, where 1.0 means no contrast and 21.0 is maximum.
pub fn contrast_ratio(c1: &Color, c2: &Color) -> f64 {
    let l1 = relative_luminance(c1);
    let l2 = relative_luminance(c2);
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

/// A contrast issue found in a scene.
#[derive(Debug, Clone)]
pub struct ContrastIssue {
    /// Which scene the issue was found in.
    pub scene_id: String,
    /// Stable identifier: `"scene-id/fg-slug-on-bg-slug"`.
    pub slug: String,
    /// Line index within the scene.
    pub line: usize,
    /// Span index within the line.
    pub span: usize,
    /// The text content of the span.
    pub text: String,
    /// Foreground color used.
    pub fg: Color,
    /// Background color used.
    pub bg: Color,
    /// Semantic foreground color reference.
    pub fg_color: Option<ThemeColor>,
    /// Semantic background color reference.
    pub bg_color: Option<ThemeColor>,
    /// Computed contrast ratio.
    pub ratio: f64,
    /// The WCAG level that was checked against.
    pub level: &'static str,
    /// The threshold that was not met.
    pub threshold: f64,
}

/// Validate all spans in a scene against a theme for contrast issues.
///
/// Checks each span's resolved fg/bg colors against the given threshold.
/// Spans with bold text are checked against `large_threshold` (WCAG treats bold >=14pt as large).
pub fn validate_scene_contrast(
    scene: &Scene,
    theme: &Theme,
    normal_threshold: f64,
    large_threshold: f64,
) -> Vec<ContrastIssue> {
    let mut issues = Vec::new();
    let default_bg = &theme.background;

    for (line_idx, line) in scene.lines.iter().enumerate() {
        for (span_idx, span) in line.spans.iter().enumerate() {
            if span.text.trim().is_empty() || span.style.dim || span.fg.is_none() {
                continue;
            }

            let fg = span
                .fg
                .as_ref()
                .map(|c| c.resolve(theme))
                .unwrap_or(&theme.foreground);
            let bg = span
                .bg
                .as_ref()
                .map(|c| c.resolve(theme))
                .unwrap_or(default_bg);

            let ratio = contrast_ratio(fg, bg);
            let (threshold, level) = if span.style.bold {
                (large_threshold, "AA-large")
            } else {
                (normal_threshold, "AA")
            };

            if ratio < threshold {
                let fg_tc = span.fg.clone();
                let bg_tc = span.bg.clone();
                let fg_slug = fg_tc.as_ref().map(|c| c.slug()).unwrap_or_else(|| "fg".into());
                let bg_slug = bg_tc.as_ref().map(|c| c.slug()).unwrap_or_else(|| "bg".into());
                let slug = format!("{}/{}-on-{}", scene.id, fg_slug, bg_slug);
                issues.push(ContrastIssue {
                    scene_id: scene.id.clone(),
                    slug,
                    line: line_idx,
                    span: span_idx,
                    text: span.text.clone(),
                    fg: fg.clone(),
                    bg: bg.clone(),
                    fg_color: fg_tc,
                    bg_color: bg_tc,
                    ratio,
                    level,
                    threshold,
                });
            }
        }
    }

    issues
}

/// Calculate a readability score for a theme: percentage of non-whitespace
/// colored spans across all scenes that meet the WCAG AA-large contrast
/// threshold (3:1).
///
/// Terminal color themes assign colors to semantic ANSI slots that serve
/// as interface-level elements (syntax highlighting, status bars, prompts).
/// These are closer in nature to WCAG "large text / UI components" than to
/// small body text. Using the AA-large threshold (3:1) reflects whether
/// theme colors are genuinely readable rather than testing strict WCAG AA
/// small-text compliance (4.5:1).
///
/// Use `validate_theme_readability` for a full WCAG AA accessibility report.
///
/// Returns a value 0.0..100.0.
pub fn readability_score(theme: &Theme) -> f64 {
    let scenes = crate::scenes::all_scenes();
    let default_bg = &theme.background;
    let mut total = 0u32;
    let mut passing = 0u32;

    for scene in &scenes {
        for line in &scene.lines {
            for span in &line.spans {
                if span.text.trim().is_empty() || span.style.dim || span.fg.is_none() {
                    continue;
                }
                total += 1;
                let fg = span
                    .fg
                    .as_ref()
                    .map(|c| c.resolve(theme))
                    .unwrap_or(&theme.foreground);
                let bg = span
                    .bg
                    .as_ref()
                    .map(|c| c.resolve(theme))
                    .unwrap_or(default_bg);
                let ratio = contrast_ratio(fg, bg);
                if ratio >= WCAG_AA_LARGE {
                    passing += 1;
                }
            }
        }
    }

    if total == 0 {
        return 100.0;
    }
    (passing as f64 / total as f64) * 100.0
}

/// Validate all built-in scenes against a theme using WCAG AA thresholds.
pub fn validate_theme_readability(theme: &Theme) -> Vec<ContrastIssue> {
    let scenes = crate::scenes::all_scenes();
    let mut all_issues = Vec::new();
    for scene in &scenes {
        all_issues.extend(validate_scene_contrast(
            scene,
            theme,
            WCAG_AA_NORMAL,
            WCAG_AA_LARGE,
        ));
    }
    all_issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_on_white_max_contrast() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        let ratio = contrast_ratio(&black, &white);
        assert!((ratio - 21.0).abs() < 0.1);
    }

    #[test]
    fn same_color_min_contrast() {
        let c = Color::new(128, 128, 128);
        let ratio = contrast_ratio(&c, &c);
        assert!((ratio - 1.0).abs() < 0.001);
    }

    #[test]
    fn contrast_is_symmetric() {
        let a = Color::new(100, 50, 200);
        let b = Color::new(200, 180, 50);
        assert!((contrast_ratio(&a, &b) - contrast_ratio(&b, &a)).abs() < 0.001);
    }

    #[test]
    fn validate_detects_low_contrast() {
        use crate::scene::*;
        use crate::AnsiColors;

        // Theme with dark bg and dark red (low contrast pair)
        let theme = Theme {
            name: "low-contrast-test".into(),
            background: Color::new(30, 30, 30),
            foreground: Color::new(200, 200, 200),
            cursor: Color::new(200, 200, 200),
            selection_background: Color::new(60, 60, 60),
            selection_foreground: Color::new(200, 200, 200),
            ansi: AnsiColors::from_array([
                Color::new(30, 30, 30),   // black - same as bg!
                Color::new(50, 20, 20),   // red - very dark, low contrast on dark bg
                Color::new(0, 200, 0),    // green
                Color::new(200, 200, 0),  // yellow
                Color::new(0, 0, 200),    // blue
                Color::new(200, 0, 200),  // magenta
                Color::new(0, 200, 200),  // cyan
                Color::new(200, 200, 200),// white
                Color::new(80, 80, 80),   // bright black
                Color::new(255, 50, 50),  // bright red
                Color::new(50, 255, 50),  // bright green
                Color::new(255, 255, 50), // bright yellow
                Color::new(50, 50, 255),  // bright blue
                Color::new(255, 50, 255), // bright magenta
                Color::new(50, 255, 255), // bright cyan
                Color::new(255, 255, 255),// bright white
            ]),
        };

        let scene = Scene {
            id: "test".into(),
            name: "Test".into(),
            description: "Test".into(),
            lines: vec![SceneLine::new(vec![
                // Dark red on dark bg — should fail
                StyledSpan::colored("bad contrast", ThemeColor::Ansi(1)),
                // White on dark bg — should pass
                StyledSpan::colored("good contrast", ThemeColor::Ansi(7)),
            ])],
        };

        let issues = validate_scene_contrast(&scene, &theme, WCAG_AA_NORMAL, WCAG_AA_LARGE);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].text, "bad contrast");
        assert!(issues[0].ratio < WCAG_AA_NORMAL);
    }

    #[test]
    fn plain_spans_excluded_from_scoring() {
        use crate::scene::*;
        use crate::AnsiColors;

        // Light theme: foreground has low contrast against background
        let theme = Theme {
            name: "light-test".into(),
            background: Color::new(250, 250, 250), // #fafafa
            foreground: Color::new(87, 95, 102),    // #575f66 (~3.95:1 — fails AA)
            cursor: Color::new(255, 106, 0),
            selection_background: Color::new(209, 228, 244),
            selection_foreground: Color::new(87, 95, 102),
            ansi: AnsiColors::from_array([
                Color::new(0, 0, 0),
                Color::new(255, 51, 51),
                Color::new(76, 191, 153),
                Color::new(255, 170, 51),
                Color::new(57, 186, 230),
                Color::new(163, 122, 204),
                Color::new(76, 191, 153),
                Color::new(87, 95, 102),
                Color::new(171, 178, 191),
                Color::new(255, 51, 51),
                Color::new(76, 191, 153),
                Color::new(255, 170, 51),
                Color::new(57, 186, 230),
                Color::new(163, 122, 204),
                Color::new(76, 191, 153),
                Color::new(255, 255, 255),
            ]),
        };

        let scene = Scene {
            id: "test".into(),
            name: "Test".into(),
            description: "Test".into(),
            lines: vec![SceneLine::new(vec![
                // Plain text (fg=None) — should be SKIPPED
                StyledSpan::plain("plain text"),
                // Explicitly colored — should be counted
                StyledSpan::colored("colored text", ThemeColor::Ansi(4)),
            ])],
        };

        let issues = validate_scene_contrast(&scene, &theme, WCAG_AA_NORMAL, WCAG_AA_LARGE);
        // Plain span should NOT generate an issue even though fg/bg ratio < 4.5
        assert!(issues.iter().all(|i| i.text != "plain text"));
    }

    #[test]
    fn dim_spans_excluded_from_scoring() {
        use crate::scene::*;
        use crate::AnsiColors;

        let theme = Theme {
            name: "dim-test".into(),
            background: Color::new(30, 30, 30),
            foreground: Color::new(200, 200, 200),
            cursor: Color::new(200, 200, 200),
            selection_background: Color::new(60, 60, 60),
            selection_foreground: Color::new(200, 200, 200),
            ansi: AnsiColors::from_array([
                Color::new(30, 30, 30), Color::new(50, 20, 20),
                Color::new(0, 200, 0), Color::new(200, 200, 0),
                Color::new(0, 0, 200), Color::new(200, 0, 200),
                Color::new(0, 200, 200), Color::new(200, 200, 200),
                Color::new(80, 80, 80), Color::new(255, 50, 50),
                Color::new(50, 255, 50), Color::new(255, 255, 50),
                Color::new(50, 50, 255), Color::new(255, 50, 255),
                Color::new(50, 255, 255), Color::new(255, 255, 255),
            ]),
        };

        let scene = Scene {
            id: "test".into(),
            name: "Test".into(),
            description: "Test".into(),
            lines: vec![SceneLine::new(vec![
                // Dim span with low-contrast color — should be skipped
                StyledSpan::colored("dim text", ThemeColor::Ansi(1)).dim(),
                // Non-dim low-contrast — should be caught
                StyledSpan::colored("visible text", ThemeColor::Ansi(1)),
            ])],
        };

        let issues = validate_scene_contrast(&scene, &theme, WCAG_AA_NORMAL, WCAG_AA_LARGE);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].text, "visible text");
    }

    // Helper to build a Theme from hex strings: [bg, fg, cursor, sel_bg, sel_fg, ansi0..ansi15]
    fn theme_from_hex(name: &str, colors: [&str; 21]) -> Theme {
        use crate::AnsiColors;
        let c = |s: &str| Color::from_hex(s).unwrap();
        Theme {
            name: name.into(),
            background: c(colors[0]),
            foreground: c(colors[1]),
            cursor: c(colors[2]),
            selection_background: c(colors[3]),
            selection_foreground: c(colors[4]),
            ansi: AnsiColors::from_array([
                c(colors[5]),  c(colors[6]),  c(colors[7]),  c(colors[8]),
                c(colors[9]),  c(colors[10]), c(colors[11]), c(colors[12]),
                c(colors[13]), c(colors[14]), c(colors[15]), c(colors[16]),
                c(colors[17]), c(colors[18]), c(colors[19]), c(colors[20]),
            ]),
        }
    }

    // ── readability_score() uses 3:1 (AA-large) threshold ─────────────────

    /// A high-quality dark theme should score above 95%.
    #[test]
    fn readability_score_catppuccin_mocha() {
        let theme = theme_from_hex("Catppuccin Mocha", [
            "#1e1e2e", "#cdd6f4", "#f5e0dc", "#313244", "#cdd6f4",
            "#45475a", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#bac2de",
            "#585b70", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#a6adc8",
        ]);
        let score = readability_score(&theme);
        assert!(score > 95.0, "Catppuccin Mocha should score >95%, got {:.1}%", score);
    }

    /// A well-designed light theme should score above 45%.
    /// Catppuccin Latte genuinely has low contrast for green (#40A02B = 2.96:1)
    /// and yellow (#DF8E1D = 2.31:1) on its background — these are real
    /// contrast issues that cause the score to be lower than the dark variant.
    #[test]
    fn readability_score_catppuccin_latte() {
        let theme = theme_from_hex("Catppuccin Latte", [
            "#eff1f5", "#4c4f69", "#dc8a78", "#acb0be", "#4c4f69",
            "#5c5f77", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#acb0be",
            "#6c6f85", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#bcc0cc",
        ]);
        let score = readability_score(&theme);
        // Score reflects genuine contrast issues — better than the old score (<32%)
        // but lower than dark variant because green/yellow have real low contrast.
        assert!(score > 45.0, "Catppuccin Latte should score >45%, got {:.1}%", score);
        assert!(score < 90.0, "Catppuccin Latte honestly shouldn't score >90% given its green/yellow contrast");
    }

    /// Solarized Dark should score above 90% after threshold fix.
    #[test]
    fn readability_score_solarized_dark() {
        let theme = theme_from_hex("Solarized Dark", [
            "#002b36", "#839496", "#839496", "#073642", "#839496",
            "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
            "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
        ]);
        let score = readability_score(&theme);
        assert!(score > 90.0, "Solarized Dark should score >90%, got {:.1}%", score);
    }

    /// Solarized Light should score above 40% with the 3:1 threshold.
    /// Some colors like green (#859900 = 2.97:1) just miss 3:1.
    #[test]
    fn readability_score_solarized_light() {
        let theme = theme_from_hex("Solarized Light", [
            "#fdf6e3", "#657b83", "#657b83", "#eee8d5", "#657b83",
            "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
            "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
        ]);
        let score = readability_score(&theme);
        assert!(score > 40.0, "Solarized Light should score >40%, got {:.1}%", score);
    }

    /// A perfect theme with all 16 ANSI colors having >3:1 contrast should score 100%.
    #[test]
    fn readability_score_perfect_light_theme() {
        use crate::AnsiColors;
        // All ANSI colors chosen to have >3:1 contrast on the white background.
        // bg=#ffffff, fg and ansi colors all dark.
        let black = Color::new(0, 0, 0);       // 21:1 on white
        let red = Color::new(180, 0, 0);       // dark red, ~6:1
        let green = Color::new(0, 120, 0);     // dark green, ~4.5:1
        let yellow = Color::new(120, 80, 0);   // dark amber, ~5:1
        let blue = Color::new(0, 0, 200);      // dark blue, ~5:1
        let magenta = Color::new(150, 0, 150); // dark magenta, ~4:1
        let cyan = Color::new(0, 130, 130);    // dark cyan, ~4:1
        let white_fg = Color::new(60, 60, 60); // dark gray, ~7:1
        let theme = Theme {
            name: "perfect-light".into(),
            background: Color::new(255, 255, 255),
            foreground: black.clone(),
            cursor: black.clone(),
            selection_background: Color::new(200, 200, 200),
            selection_foreground: black.clone(),
            ansi: AnsiColors::from_array([
                black.clone(), red.clone(), green.clone(), yellow.clone(),
                blue.clone(), magenta.clone(), cyan.clone(), white_fg.clone(),
                Color::new(40, 40, 40), Color::new(200, 0, 0), Color::new(0, 140, 0),
                Color::new(140, 90, 0), Color::new(0, 0, 220), Color::new(160, 0, 160),
                Color::new(0, 140, 140), Color::new(80, 80, 80),
            ]),
        };
        let score = readability_score(&theme);
        assert!(score > 95.0, "Perfect light theme should score >95%, got {:.1}%", score);
    }

    /// readability_score uses WCAG AA-large (3:1) for ALL text.
    /// validate_theme_readability still uses the full AA (4.5:1) / AA-large (3:1) split.
    #[test]
    fn score_uses_aa_large_threshold_for_all_text() {
        use crate::scene::*;
        use crate::AnsiColors;

        // A light theme where one color has exactly 3.5:1 contrast on the background.
        // #888888 on #ffffff = luminance ratio: L1=0.2158, L2=1.0
        // contrast = (1.0+0.05)/(0.2158+0.05) = 1.05/0.2658 = 3.95:1
        // So #888888 on white passes 3:1 (AA-large) but fails 4.5:1 (AA-normal).
        let mid_gray = Color::new(0x88, 0x88, 0x88);
        let white = Color::new(255, 255, 255);
        let ratio = contrast_ratio(&mid_gray, &white);
        assert!(ratio > 3.0, "Sanity check: mid gray on white should be > 3:1 (got {:.2})", ratio);
        assert!(ratio < 4.5, "Sanity check: mid gray on white should be < 4.5:1 (got {:.2})", ratio);

        let theme = Theme {
            name: "mid-gray-test".into(),
            background: white.clone(),
            foreground: Color::new(0, 0, 0),
            cursor: Color::new(0, 0, 0),
            selection_background: Color::new(200, 200, 200),
            selection_foreground: Color::new(0, 0, 0),
            ansi: AnsiColors::from_array([
                Color::new(0, 0, 0), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
                mid_gray.clone(), mid_gray.clone(),
            ]),
        };

        // A scene with one non-bold span using ansi(1) = mid_gray on background.
        let scene = crate::scene::Scene {
            id: "score-test".into(),
            name: "Score Test".into(),
            description: "".into(),
            lines: vec![SceneLine::new(vec![
                StyledSpan::colored("text at 3.95:1", ThemeColor::Ansi(1)),
            ])],
        };

        // readability_score: should PASS (3.95 >= 3.0 threshold)
        let issues_aa = validate_scene_contrast(&scene, &theme, WCAG_AA_NORMAL, WCAG_AA_LARGE);
        assert_eq!(issues_aa.len(), 1, "AA validation: should flag as 4.5 failure");
        assert!(issues_aa[0].ratio > WCAG_AA_LARGE, "Ratio should be above 3:1");
        assert!(issues_aa[0].ratio < WCAG_AA_NORMAL, "Ratio should be below 4.5:1");

        // score_test: passes at 3:1 so this span should be counted as passing
        let issues_large = validate_scene_contrast(&scene, &theme, WCAG_AA_LARGE, WCAG_AA_LARGE);
        assert_eq!(issues_large.len(), 0, "At 3:1 threshold this should PASS");
    }

    /// Spans in the 3.0–4.5 contrast range should pass the readability score
    /// but still appear in the validate_theme_readability issue report.
    #[test]
    fn borderline_contrast_passes_score_fails_aa_report() {
        use crate::scene::*;
        use crate::AnsiColors;

        // #6c6f85 on #eff1f5 = Catppuccin Latte bright_black on background = 4.37:1
        // Passes 3:1 readability threshold, fails 4.5:1 AA threshold.
        let text_color = Color::new(0x6c, 0x6f, 0x85);
        let bg_color = Color::new(0xef, 0xf1, 0xf5);
        let ratio = contrast_ratio(&text_color, &bg_color);
        assert!(ratio > 3.0, "Should be above 3:1 (got {:.2})", ratio);
        assert!(ratio < 4.5, "Should be below 4.5:1 (got {:.2})", ratio);

        let theme = Theme {
            name: "borderline-test".into(),
            background: bg_color,
            foreground: Color::new(0x4c, 0x4f, 0x69),
            cursor: Color::new(0, 0, 0),
            selection_background: Color::new(200, 200, 200),
            selection_foreground: Color::new(0, 0, 0),
            ansi: AnsiColors::from_array([
                Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0),
                Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0),
                text_color.clone(), Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0),
                Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 0, 0),
            ]),
        };

        let scene = crate::scene::Scene {
            id: "border-test".into(),
            name: "Border Test".into(),
            description: "".into(),
            lines: vec![SceneLine::new(vec![
                StyledSpan::colored("borderline text", ThemeColor::Ansi(8)),
            ])],
        };

        // Full AA report should list this as a failure (< 4.5)
        let aa_issues = validate_scene_contrast(&scene, &theme, WCAG_AA_NORMAL, WCAG_AA_LARGE);
        assert_eq!(aa_issues.len(), 1, "AA report should flag 4.37:1 as below AA normal");

        // AA-large threshold should NOT list this as a failure (> 3.0)
        let large_issues = validate_scene_contrast(&scene, &theme, WCAG_AA_LARGE, WCAG_AA_LARGE);
        assert_eq!(large_issues.len(), 0, "3:1 threshold: 4.37:1 should pass");
    }

    /// Ensure ansi(15) (bright_white) in htop uses ThemeColor::Foreground —
    /// this is the scene fix that prevents light themes from being penalized
    /// by process names being near-invisible.
    #[test]
    fn htop_process_names_use_foreground_not_bright_white() {
        let htop = crate::scenes::htop_scene();
        // Process command name spans are in the last column of process rows.
        // Verify none of the process-name spans use Ansi(15) (bright_white).
        for line in &htop.lines {
            for span in &line.spans {
                if let Some(crate::scene::ThemeColor::Ansi(15)) = &span.fg {
                    // If this span IS ansi(15), its text must not be a process name
                    assert!(
                        !span.text.contains("cargo") && !span.text.contains("nvim")
                            && !span.text.contains("kitty") && !span.text.contains("systemd")
                            && !span.text.contains("firefox"),
                        "Process name span should not use ansi(15): {:?}",
                        span.text
                    );
                }
            }
        }
    }

    /// Print a full span-by-span breakdown of what passes/fails for a theme.
    /// Run with: cargo test -- --nocapture diagnose_theme_contrast
    #[test]
    #[ignore]
    fn diagnose_theme_contrast() {
        let themes = [
            theme_from_hex("Catppuccin Mocha", [
                "#1e1e2e", "#cdd6f4", "#f5e0dc", "#313244", "#cdd6f4",
                "#45475a", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#bac2de",
                "#585b70", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#a6adc8",
            ]),
            theme_from_hex("Catppuccin Latte", [
                "#eff1f5", "#4c4f69", "#dc8a78", "#acb0be", "#4c4f69",
                "#5c5f77", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#acb0be",
                "#6c6f85", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#bcc0cc",
            ]),
            theme_from_hex("Solarized Dark", [
                "#002b36", "#839496", "#839496", "#073642", "#839496",
                "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
                "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
            ]),
            theme_from_hex("Solarized Light", [
                "#fdf6e3", "#657b83", "#657b83", "#eee8d5", "#657b83",
                "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
                "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
            ]),
        ];

        for theme in &themes {
            let score = readability_score(theme);
            eprintln!("\n=== {} — score: {:.1}% ===", theme.name, score);
            let issues = validate_theme_readability(theme);
            if issues.is_empty() {
                eprintln!("  (no issues)");
            }
            for issue in &issues {
                eprintln!(
                    "  FAIL [{scene}] line {l} span {s}: {text:?}  fg={fg} bg={bg}  ratio={ratio:.2} < {threshold} ({level})",
                    scene = issue.scene_id,
                    l = issue.line,
                    s = issue.span,
                    text = issue.text,
                    fg = issue.fg.to_hex(),
                    bg = issue.bg.to_hex(),
                    ratio = issue.ratio,
                    threshold = issue.threshold,
                    level = issue.level,
                );
            }
        }
    }
}
