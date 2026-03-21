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
            if span.text.trim().is_empty() {
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
/// spans across all scenes that pass WCAG AA contrast thresholds.
/// Returns a value 0.0..100.0.
pub fn readability_score(theme: &Theme) -> f64 {
    let scenes = crate::scenes::all_scenes();
    let default_bg = &theme.background;
    let mut total = 0u32;
    let mut passing = 0u32;

    for scene in &scenes {
        for line in &scene.lines {
            for span in &line.spans {
                if span.text.trim().is_empty() {
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
                let threshold = if span.style.bold {
                    WCAG_AA_LARGE
                } else {
                    WCAG_AA_NORMAL
                };
                if ratio >= threshold {
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
}
