//! Contrast calculation and readability validation.
//!
//! Provides two contrast algorithms:
//!
//! - **WCAG 2.x contrast ratio** — the established W3C standard, used in
//!   `validate_theme_readability` for accessibility compliance reporting.
//!   Symmetric (polarity-agnostic), which under-estimates perceived contrast
//!   for dark text on light backgrounds.
//!
//! - **APCA (Accessible Perceptual Contrast Algorithm)** — the successor
//!   algorithm designed for WCAG 3.0. Accounts for polarity (dark-on-light
//!   vs light-on-dark), producing more accurate perceived-contrast values.
//!   Used in `readability_score` because it correctly models that saturated
//!   dark text on light backgrounds (e.g. terminal green, cyan, yellow) is
//!   more readable than WCAG 2.x would predict.

use crate::scene::{Scene, ThemeColor};
use crate::term_output::{TermColor, TermOutput};
use crate::{Color, Theme};

/// Minimum contrast ratio for WCAG AA normal text.
pub const WCAG_AA_NORMAL: f64 = 4.5;
/// Minimum contrast ratio for WCAG AA large text (bold >=14pt or normal >=18pt).
pub const WCAG_AA_LARGE: f64 = 3.0;
/// Minimum contrast ratio for WCAG AAA normal text.
pub const WCAG_AAA_NORMAL: f64 = 7.0;

/// Minimum APCA lightness contrast (|Lc|) for the readability score.
///
/// APCA level 30 corresponds to "non-text / spot text / large bold" in the
/// APCA guidelines. Terminal ANSI colors are interface-level elements
/// (syntax highlight, prompts, status indicators) closer to UI components
/// than to dense body text, so this threshold is appropriate.
///
/// Empirically, Lc ≥ 30 passes all colors that users perceive as "clearly
/// readable" in popular light themes (Catppuccin Latte, Solarized Light)
/// while still catching genuinely poor contrast like bright-white-on-white.
pub const APCA_MIN_READABLE: f64 = 30.0;

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

/// Calculate WCAG 2.x contrast ratio between two colors.
/// Returns a value >= 1.0, where 1.0 means no contrast and 21.0 is maximum.
/// Symmetric: `contrast_ratio(a, b) == contrast_ratio(b, a)`.
pub fn contrast_ratio(c1: &Color, c2: &Color) -> f64 {
    let l1 = relative_luminance(c1);
    let l2 = relative_luminance(c2);
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Calculate APCA lightness contrast (Lc) between text and background.
///
/// Returns a value in approximately \[-108, +108\]:
/// - **Positive** Lc → dark text on light background (normal polarity)
/// - **Negative** Lc → light text on dark background (reverse polarity)
/// - **Zero** → no perceptible contrast
///
/// Unlike WCAG 2.x, APCA is **polarity-aware**: it correctly models that
/// dark text on light backgrounds has higher perceived contrast than the
/// symmetric WCAG ratio would predict.  This makes it more accurate for
/// evaluating light terminal themes where saturated ANSI colors (green,
/// cyan, yellow) are used as dark text on a bright background.
///
/// Reference: APCA-W3 by Andrew Somers / Myndex Research
pub fn apca_contrast(text: &Color, bg: &Color) -> f64 {
    // APCA-W3 constants (v0.1.9)
    const NORM_BG: f64 = 0.56;
    const NORM_TXT: f64 = 0.57;
    const REV_TXT: f64 = 0.62;
    const REV_BG: f64 = 0.65;
    const BLK_THRS: f64 = 0.022;
    const BLK_CLMP: f64 = 1.414;
    const SCALE_BOW: f64 = 1.14;
    const SCALE_WOB: f64 = 1.14;
    const LO_BOW_OFFSET: f64 = 0.027;
    const LO_WOB_OFFSET: f64 = 0.027;
    const LO_CLIP: f64 = 0.1;
    const DELTA_Y_MIN: f64 = 0.0005;

    let mut txt_y = relative_luminance(text);
    let mut bg_y = relative_luminance(bg);

    // Soft-clamp near-black luminance
    if txt_y <= BLK_THRS {
        txt_y += (BLK_THRS - txt_y).powf(BLK_CLMP);
    }
    if bg_y <= BLK_THRS {
        bg_y += (BLK_THRS - bg_y).powf(BLK_CLMP);
    }

    if (bg_y - txt_y).abs() < DELTA_Y_MIN {
        return 0.0;
    }

    if bg_y > txt_y {
        // Normal polarity: dark text on light background
        let sapc = (bg_y.powf(NORM_BG) - txt_y.powf(NORM_TXT)) * SCALE_BOW;
        if sapc < LO_CLIP {
            0.0
        } else {
            (sapc - LO_BOW_OFFSET) * 100.0
        }
    } else {
        // Reverse polarity: light text on dark background
        let sapc = (bg_y.powf(REV_BG) - txt_y.powf(REV_TXT)) * SCALE_WOB;
        if sapc > -LO_CLIP {
            0.0
        } else {
            (sapc + LO_WOB_OFFSET) * 100.0
        }
    }
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
/// Uses APCA (|Lc| ≥ [`APCA_MIN_READABLE`]) for pass/fail detection,
/// matching the algorithm used by [`readability_score`]. The WCAG 2.x
/// contrast ratio is still stored in each issue for informational display.
pub fn validate_scene_contrast(
    scene: &Scene,
    theme: &Theme,
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

            let lc = apca_contrast(fg, bg).abs();
            if lc < APCA_MIN_READABLE {
                let ratio = contrast_ratio(fg, bg);
                let level = if span.style.bold { "AA-large" } else { "AA" };
                let threshold = APCA_MIN_READABLE;

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

/// Calculate a readability score for a theme using **APCA** (Accessible
/// Perceptual Contrast Algorithm).
///
/// Returns the percentage (0.0..100.0) of non-whitespace colored spans
/// across all built-in scenes that meet the [`APCA_MIN_READABLE`] threshold
/// (|Lc| ≥ 30).
///
/// APCA is polarity-aware, correctly modeling that dark text on light
/// backgrounds has higher perceived contrast than WCAG 2.x predicts.
///
/// Uses the same algorithm as [`validate_theme_readability`], ensuring
/// the score and issue count are always consistent.
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
                let lc = apca_contrast(fg, bg).abs();
                if lc >= APCA_MIN_READABLE {
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

/// Validate all built-in scenes against a theme using APCA thresholds.
pub fn validate_theme_readability(theme: &Theme) -> Vec<ContrastIssue> {
    let scenes = crate::scenes::all_scenes();
    let mut all_issues = Vec::new();
    for scene in &scenes {
        all_issues.extend(validate_scene_contrast(scene, theme));
    }
    all_issues
}

// ── TermOutput contrast validation ────────────────────────────────────

/// A contrast issue found in a TermOutput fixture.
#[derive(Debug, Clone)]
pub struct TermContrastIssue {
    /// Fixture ID where the issue was found.
    pub fixture_id: String,
    /// Line index within the fixture.
    pub line: usize,
    /// Span index within the line.
    pub span: usize,
    /// The text content of the span.
    pub text: String,
    /// Resolved foreground color.
    pub fg: Color,
    /// Resolved background color.
    pub bg: Color,
    /// The TermColor variant for the foreground.
    pub fg_term: TermColor,
    /// The TermColor variant for the background.
    pub bg_term: TermColor,
    /// Computed WCAG 2.x contrast ratio (informational).
    pub ratio: f64,
    /// APCA Lc threshold that was not met.
    pub threshold: f64,
}

/// Returns true if a TermColor is theme-independent (fixed RGB value).
fn is_fixed_color(tc: &TermColor) -> bool {
    matches!(tc, TermColor::Indexed(_) | TermColor::Rgb(_, _, _))
}

/// Validate all spans in a TermOutput fixture against a theme for contrast issues.
///
/// Uses APCA (|Lc| >= [`APCA_MIN_READABLE`]) for pass/fail. Skips:
/// - Empty/whitespace-only spans
/// - Dim spans (intentionally low contrast)
/// - Spans where both fg and bg are fixed colors (theme-independent)
pub fn validate_term_output_contrast(
    output: &TermOutput,
    theme: &Theme,
) -> Vec<TermContrastIssue> {
    let mut issues = Vec::new();

    for (line_idx, line) in output.lines.iter().enumerate() {
        for (span_idx, span) in line.spans.iter().enumerate() {
            if span.text.trim().is_empty() || span.dim {
                continue;
            }

            // Skip if both fg and bg are fixed (theme-independent)
            if is_fixed_color(&span.fg) && is_fixed_color(&span.bg) {
                continue;
            }

            // Skip Default/Default — that's just theme fg on theme bg, not
            // something the fixture controls
            if span.fg == TermColor::Default && span.bg == TermColor::Default {
                continue;
            }

            let fg = span.fg.resolve_with_theme(theme, &theme.foreground);
            let bg = span.bg.resolve_with_theme(theme, &theme.background);

            let lc = apca_contrast(&fg, &bg).abs();
            if lc < APCA_MIN_READABLE {
                let ratio = contrast_ratio(&fg, &bg);
                issues.push(TermContrastIssue {
                    fixture_id: output.id.clone(),
                    line: line_idx,
                    span: span_idx,
                    text: span.text.clone(),
                    fg: fg.clone(),
                    bg: bg.clone(),
                    fg_term: span.fg,
                    bg_term: span.bg,
                    ratio,
                    threshold: APCA_MIN_READABLE,
                });
            }
        }
    }

    issues
}

/// Validate multiple TermOutput fixtures against a theme.
pub fn validate_fixtures_contrast(
    fixtures: &[TermOutput],
    theme: &Theme,
) -> Vec<TermContrastIssue> {
    fixtures
        .iter()
        .flat_map(|f| validate_term_output_contrast(f, theme))
        .collect()
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

        let issues = validate_scene_contrast(&scene, &theme);
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

        let issues = validate_scene_contrast(&scene, &theme);
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

        let issues = validate_scene_contrast(&scene, &theme);
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

    // ── APCA contrast tests ──────────────────────────────────────────────

    #[test]
    fn apca_black_on_white() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        // Dark text on light bg → positive Lc, should be ~107
        let lc = apca_contrast(&black, &white);
        assert!(lc > 100.0, "Black on white APCA Lc should be >100, got {:.1}", lc);
    }

    #[test]
    fn apca_white_on_black() {
        let black = Color::new(0, 0, 0);
        let white = Color::new(255, 255, 255);
        // Light text on dark bg → negative Lc
        let lc = apca_contrast(&white, &black);
        assert!(lc < -100.0, "White on black APCA Lc should be < -100, got {:.1}", lc);
    }

    #[test]
    fn apca_same_color_zero() {
        let c = Color::new(128, 128, 128);
        let lc = apca_contrast(&c, &c);
        assert!(lc.abs() < 1.0, "Same color should give ~0 Lc, got {:.1}", lc);
    }

    /// Verify APCA polarity: dark text on light bg has HIGHER perceived
    /// contrast than WCAG ratio alone suggests. This is the key property
    /// that fixes light theme scoring.
    #[test]
    fn apca_polarity_favors_dark_on_light() {
        // Catppuccin Latte green: WCAG 2.96:1 (fails WCAG AA-large 3.0)
        // but APCA Lc ~50 (passes APCA_MIN_READABLE = 30)
        let green = Color::new(0x40, 0xa0, 0x2b);
        let bg = Color::new(0xef, 0xf1, 0xf5);
        let lc = apca_contrast(&green, &bg);
        assert!(lc > 0.0, "Dark-on-light should give positive Lc");
        assert!(lc >= APCA_MIN_READABLE, "Latte green (WCAG 2.96:1) should pass APCA at Lc {:.1}", lc);
    }

    /// Yellow on light bg: WCAG 2.31:1 but visually readable.
    #[test]
    fn apca_yellow_on_light_bg_passes() {
        let yellow = Color::new(0xdf, 0x8e, 0x1d);
        let bg = Color::new(0xef, 0xf1, 0xf5);
        let lc = apca_contrast(&yellow, &bg);
        assert!(lc >= APCA_MIN_READABLE, "Latte yellow should pass APCA at Lc {:.1}", lc);
    }

    /// Near-white on white should genuinely fail (truly unreadable).
    #[test]
    fn apca_near_white_on_white_fails() {
        let near_white = Color::new(0xbc, 0xc0, 0xcc);
        let white = Color::new(0xef, 0xf1, 0xf5);
        let lc = apca_contrast(&near_white, &white);
        assert!(lc.abs() < APCA_MIN_READABLE, "Near-white on white should fail APCA at Lc {:.1}", lc);
    }

    // ── readability_score() — uses WCAG 2.x (consistent with issues) ────

    /// A high-quality dark theme should score above 85%.
    #[test]
    fn readability_score_catppuccin_mocha() {
        let theme = theme_from_hex("Catppuccin Mocha", [
            "#1e1e2e", "#cdd6f4", "#f5e0dc", "#313244", "#cdd6f4",
            "#45475a", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#bac2de",
            "#585b70", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#a6adc8",
        ]);
        let score = readability_score(&theme);
        assert!(score > 85.0, "Catppuccin Mocha should score >85%, got {:.1}%", score);
    }

    /// Catppuccin Latte should have a reasonable score.
    #[test]
    fn readability_score_catppuccin_latte() {
        let theme = theme_from_hex("Catppuccin Latte", [
            "#eff1f5", "#4c4f69", "#dc8a78", "#acb0be", "#4c4f69",
            "#5c5f77", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#acb0be",
            "#6c6f85", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#bcc0cc",
        ]);
        let score = readability_score(&theme);
        assert!(score > 50.0, "Catppuccin Latte should score >50%, got {:.1}%", score);
    }

    /// Solarized Dark should score reasonably.
    #[test]
    fn readability_score_solarized_dark() {
        let theme = theme_from_hex("Solarized Dark", [
            "#002b36", "#839496", "#839496", "#073642", "#839496",
            "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
            "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
        ]);
        let score = readability_score(&theme);
        assert!(score > 50.0, "Solarized Dark should score >50%, got {:.1}%", score);
    }

    /// Solarized Light score.
    #[test]
    fn readability_score_solarized_light() {
        let theme = theme_from_hex("Solarized Light", [
            "#fdf6e3", "#657b83", "#657b83", "#eee8d5", "#657b83",
            "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
            "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
        ]);
        let score = readability_score(&theme);
        // With WCAG, light themes score lower due to symmetric ratio
        assert!(score > 40.0, "Solarized Light should score >40%, got {:.1}%", score);
    }

    /// Readability score should be consistent with issue count:
    /// score = (total_spans - issue_spans) / total_spans * 100.
    #[test]
    fn readability_score_consistent_with_issues() {
        let theme = theme_from_hex("Solarized Light", [
            "#fdf6e3", "#657b83", "#657b83", "#eee8d5", "#657b83",
            "#073642", "#dc322f", "#859900", "#b58900", "#268bd2", "#d33682", "#2aa198", "#eee8d5",
            "#002b36", "#cb4b16", "#586e75", "#657b83", "#839496", "#6c71c4", "#93a1a1", "#fdf6e3",
        ]);
        let score = readability_score(&theme);
        let issues = validate_theme_readability(&theme);
        // If there are issues, score must be < 100
        if !issues.is_empty() {
            assert!(score < 100.0, "Score should be <100% when there are {} issues", issues.len());
        }
    }

    /// A perfect theme with all dark ANSI colors on white background should
    /// score near 100%.
    #[test]
    fn readability_score_perfect_light_theme() {
        use crate::AnsiColors;
        let black = Color::new(0, 0, 0);
        let red = Color::new(180, 0, 0);
        let green = Color::new(0, 120, 0);
        let yellow = Color::new(120, 80, 0);
        let blue = Color::new(0, 0, 200);
        let magenta = Color::new(150, 0, 150);
        let cyan = Color::new(0, 130, 130);
        let white_fg = Color::new(60, 60, 60);
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

    /// Light and dark variants may differ more with WCAG than APCA, but
    /// the gap should still be bounded.
    #[test]
    fn light_dark_score_gap_bounded() {
        let mocha = theme_from_hex("Catppuccin Mocha", [
            "#1e1e2e", "#cdd6f4", "#f5e0dc", "#313244", "#cdd6f4",
            "#45475a", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#bac2de",
            "#585b70", "#f38ba8", "#a6e3a1", "#f9e2af", "#89b4fa", "#cba6f7", "#89dceb", "#a6adc8",
        ]);
        let latte = theme_from_hex("Catppuccin Latte", [
            "#eff1f5", "#4c4f69", "#dc8a78", "#acb0be", "#4c4f69",
            "#5c5f77", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#acb0be",
            "#6c6f85", "#d20f39", "#40a02b", "#df8e1d", "#1e66f5", "#8839ef", "#179299", "#bcc0cc",
        ]);
        let mocha_score = readability_score(&mocha);
        let latte_score = readability_score(&latte);
        let gap = (mocha_score - latte_score).abs();
        assert!(
            gap < 40.0,
            "Catppuccin Mocha ({:.1}%) and Latte ({:.1}%) should be within 40pp (gap: {:.1}pp)",
            mocha_score, latte_score, gap
        );
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

    // ── TermOutput contrast validation tests ─────────────────────────

    fn make_term_output(id: &str, lines: Vec<crate::term_output::TermLine>) -> TermOutput {
        TermOutput {
            id: id.into(),
            name: id.into(),
            cols: 80,
            rows: 24,
            lines,
        }
    }

    fn dark_theme() -> Theme {
        use crate::AnsiColors;
        Theme {
            name: "dark-test".into(),
            background: Color::new(30, 30, 30),
            foreground: Color::new(200, 200, 200),
            cursor: Color::new(200, 200, 200),
            selection_background: Color::new(60, 60, 60),
            selection_foreground: Color::new(200, 200, 200),
            ansi: AnsiColors::from_array([
                Color::new(30, 30, 30),   // 0 black — same as bg!
                Color::new(50, 20, 20),   // 1 red — very dark
                Color::new(0, 200, 0),    // 2 green
                Color::new(200, 200, 0),  // 3 yellow
                Color::new(0, 0, 200),    // 4 blue
                Color::new(200, 0, 200),  // 5 magenta
                Color::new(0, 200, 200),  // 6 cyan
                Color::new(200, 200, 200),// 7 white
                Color::new(80, 80, 80),   // 8 bright black
                Color::new(255, 50, 50),  // 9 bright red
                Color::new(50, 255, 50),  // 10 bright green
                Color::new(255, 255, 50), // 11 bright yellow
                Color::new(50, 50, 255),  // 12 bright blue
                Color::new(255, 50, 255), // 13 bright magenta
                Color::new(50, 255, 255), // 14 bright cyan
                Color::new(255, 255, 255),// 15 bright white
            ]),
        }
    }

    #[test]
    fn term_contrast_detects_low_contrast_ansi() {
        use crate::term_output::*;
        let output = make_term_output("test", vec![
            TermLine::new(vec![
                // ANSI red (very dark) on default bg (dark) — should fail
                TermSpan {
                    text: "bad contrast".into(),
                    fg: TermColor::Ansi(1),
                    bg: TermColor::Default,
                    bold: false, italic: false, dim: false, underline: false,
                },
                // ANSI white on default bg — should pass
                TermSpan {
                    text: "good contrast".into(),
                    fg: TermColor::Ansi(7),
                    bg: TermColor::Default,
                    bold: false, italic: false, dim: false, underline: false,
                },
            ]),
        ]);
        let theme = dark_theme();
        let issues = validate_term_output_contrast(&output, &theme);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].text, "bad contrast");
        assert_eq!(issues[0].fixture_id, "test");
    }

    #[test]
    fn term_contrast_skips_fixed_pairs() {
        use crate::term_output::*;
        let output = make_term_output("test", vec![
            TermLine::new(vec![
                // Both Rgb — theme-independent, should be skipped
                TermSpan {
                    text: "fixed colors".into(),
                    fg: TermColor::Rgb(30, 30, 30),
                    bg: TermColor::Rgb(31, 31, 31),
                    bold: false, italic: false, dim: false, underline: false,
                },
                // Both Indexed — theme-independent, should be skipped
                TermSpan {
                    text: "indexed colors".into(),
                    fg: TermColor::Indexed(232),
                    bg: TermColor::Indexed(233),
                    bold: false, italic: false, dim: false, underline: false,
                },
            ]),
        ]);
        let theme = dark_theme();
        let issues = validate_term_output_contrast(&output, &theme);
        assert!(issues.is_empty(), "Fixed color pairs should not generate issues");
    }

    #[test]
    fn term_contrast_validates_ansi_on_ansi_bg() {
        use crate::term_output::*;
        // ANSI black (0) on ANSI red (1) — both theme-dependent, should be validated
        // In dark_theme, ansi[0]=(30,30,30) on ansi[1]=(50,20,20) — very low contrast
        let output = make_term_output("test", vec![
            TermLine::new(vec![
                TermSpan {
                    text: "ansi on ansi".into(),
                    fg: TermColor::Ansi(0),
                    bg: TermColor::Ansi(1),
                    bold: false, italic: false, dim: false, underline: false,
                },
            ]),
        ]);
        let theme = dark_theme();
        let issues = validate_term_output_contrast(&output, &theme);
        assert_eq!(issues.len(), 1, "Ansi-on-Ansi with low contrast should be flagged");
        assert_eq!(issues[0].text, "ansi on ansi");
    }

    #[test]
    fn term_contrast_validates_fixed_fg_on_theme_bg() {
        use crate::term_output::*;
        // Dark RGB color on default (dark) background — should flag
        let output = make_term_output("test", vec![
            TermLine::new(vec![
                TermSpan {
                    text: "dark fixed on dark bg".into(),
                    fg: TermColor::Rgb(35, 35, 35),
                    bg: TermColor::Default,
                    bold: false, italic: false, dim: false, underline: false,
                },
            ]),
        ]);
        let theme = dark_theme();
        let issues = validate_term_output_contrast(&output, &theme);
        assert_eq!(issues.len(), 1, "Fixed dark color on dark theme bg should be flagged");
    }

    #[test]
    fn term_contrast_skips_dim_spans() {
        use crate::term_output::*;
        let output = make_term_output("test", vec![
            TermLine::new(vec![
                TermSpan {
                    text: "dim text".into(),
                    fg: TermColor::Ansi(1),
                    bg: TermColor::Default,
                    bold: false, italic: false, dim: true, underline: false,
                },
            ]),
        ]);
        let theme = dark_theme();
        let issues = validate_term_output_contrast(&output, &theme);
        assert!(issues.is_empty(), "Dim spans should be skipped");
    }

    #[test]
    fn term_contrast_skips_default_on_default() {
        use crate::term_output::*;
        let output = make_term_output("test", vec![
            TermLine::new(vec![
                TermSpan::plain("plain text"),
            ]),
        ]);
        let theme = dark_theme();
        let issues = validate_term_output_contrast(&output, &theme);
        assert!(issues.is_empty(), "Default/Default spans should be skipped");
    }

    #[test]
    fn validate_fixtures_contrast_aggregates() {
        use crate::term_output::*;
        let f1 = make_term_output("f1", vec![
            TermLine::new(vec![TermSpan {
                text: "bad".into(),
                fg: TermColor::Ansi(1),
                bg: TermColor::Default,
                bold: false, italic: false, dim: false, underline: false,
            }]),
        ]);
        let f2 = make_term_output("f2", vec![
            TermLine::new(vec![TermSpan {
                text: "also bad".into(),
                fg: TermColor::Ansi(0),
                bg: TermColor::Default,
                bold: false, italic: false, dim: false, underline: false,
            }]),
        ]);
        let theme = dark_theme();
        let issues = validate_fixtures_contrast(&[f1, f2], &theme);
        assert!(issues.len() >= 2, "Should aggregate issues from multiple fixtures");
        assert!(issues.iter().any(|i| i.fixture_id == "f1"));
        assert!(issues.iter().any(|i| i.fixture_id == "f2"));
    }
}
