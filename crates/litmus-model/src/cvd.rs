//! Color Vision Deficiency (CVD) simulation using Machado 2009 matrices.
//!
//! Simulates how themes appear under protanopia, deuteranopia, and tritanopia.

use crate::{Color, Theme};
use serde::{Deserialize, Serialize};

/// Types of color vision deficiency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CvdType {
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

impl CvdType {
    /// All CVD types for iteration.
    pub fn all() -> &'static [CvdType] {
        &[CvdType::Protanopia, CvdType::Deuteranopia, CvdType::Tritanopia]
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            CvdType::Protanopia => "Protanopia",
            CvdType::Deuteranopia => "Deuteranopia",
            CvdType::Tritanopia => "Tritanopia",
        }
    }

    /// Short description of the condition.
    pub fn description(self) -> &'static str {
        match self {
            CvdType::Protanopia => "Reduced sensitivity to red light (~1% of males)",
            CvdType::Deuteranopia => "Reduced sensitivity to green light (~1% of males)",
            CvdType::Tritanopia => "Reduced sensitivity to blue light (very rare)",
        }
    }
}

/// Machado 2009 severity=1.0 simulation matrices (row-major 3x3).
/// These transform linear RGB to simulate how colors appear with each CVD type.
const PROTANOPIA_MATRIX: [[f64; 3]; 3] = [
    [0.152286, 1.052583, -0.204868],
    [0.114503, 0.786281,  0.099216],
    [-0.003882, -0.048116, 1.051998],
];

const DEUTERANOPIA_MATRIX: [[f64; 3]; 3] = [
    [0.367322, 0.860646, -0.227968],
    [0.280085, 0.672501,  0.047413],
    [-0.011820, 0.042940,  0.968881],
];

const TRITANOPIA_MATRIX: [[f64; 3]; 3] = [
    [1.255528, -0.076749, -0.178779],
    [-0.078411, 0.930809,  0.147602],
    [0.004733, 0.691367,  0.303900],
];

/// Convert sRGB component (0-255) to linear RGB (0.0-1.0).
fn srgb_to_linear(c: u8) -> f64 {
    let s = c as f64 / 255.0;
    if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear RGB (0.0-1.0) to sRGB component (0-255).
fn linear_to_srgb(c: f64) -> u8 {
    let c = c.clamp(0.0, 1.0);
    let s = if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (s * 255.0).round() as u8
}

/// Apply a 3x3 matrix to a linear RGB triplet.
fn apply_matrix(matrix: &[[f64; 3]; 3], r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    (
        matrix[0][0] * r + matrix[0][1] * g + matrix[0][2] * b,
        matrix[1][0] * r + matrix[1][1] * g + matrix[1][2] * b,
        matrix[2][0] * r + matrix[2][1] * g + matrix[2][2] * b,
    )
}

/// Simulate how a single color appears under a given CVD type.
pub fn simulate_cvd(color: &Color, cvd_type: CvdType) -> Color {
    let r = srgb_to_linear(color.r);
    let g = srgb_to_linear(color.g);
    let b = srgb_to_linear(color.b);

    let matrix = match cvd_type {
        CvdType::Protanopia => &PROTANOPIA_MATRIX,
        CvdType::Deuteranopia => &DEUTERANOPIA_MATRIX,
        CvdType::Tritanopia => &TRITANOPIA_MATRIX,
    };

    let (nr, ng, nb) = apply_matrix(matrix, r, g, b);

    Color::new(linear_to_srgb(nr), linear_to_srgb(ng), linear_to_srgb(nb))
}

/// Simulate how an entire theme appears under a given CVD type.
pub fn simulate_theme(theme: &Theme, cvd_type: CvdType) -> Theme {
    let sim = |c: &Color| simulate_cvd(c, cvd_type);
    Theme {
        name: theme.name.clone(),
        background: sim(&theme.background),
        foreground: sim(&theme.foreground),
        cursor: sim(&theme.cursor),
        selection_background: sim(&theme.selection_background),
        selection_foreground: sim(&theme.selection_foreground),
        ansi: crate::AnsiColors::from_array([
            sim(&theme.ansi.black),
            sim(&theme.ansi.red),
            sim(&theme.ansi.green),
            sim(&theme.ansi.yellow),
            sim(&theme.ansi.blue),
            sim(&theme.ansi.magenta),
            sim(&theme.ansi.cyan),
            sim(&theme.ansi.white),
            sim(&theme.ansi.bright_black),
            sim(&theme.ansi.bright_red),
            sim(&theme.ansi.bright_green),
            sim(&theme.ansi.bright_yellow),
            sim(&theme.ansi.bright_blue),
            sim(&theme.ansi.bright_magenta),
            sim(&theme.ansi.bright_cyan),
            sim(&theme.ansi.bright_white),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srgb_linear_round_trip() {
        for v in [0u8, 1, 10, 50, 128, 200, 255] {
            let linear = srgb_to_linear(v);
            let back = linear_to_srgb(linear);
            assert!((v as i16 - back as i16).abs() <= 1, "round trip failed for {v}: got {back}");
        }
    }

    #[test]
    fn black_stays_black() {
        let black = Color::new(0, 0, 0);
        for cvd in CvdType::all() {
            let result = simulate_cvd(&black, *cvd);
            assert_eq!(result, Color::new(0, 0, 0), "black should stay black under {cvd:?}");
        }
    }

    #[test]
    fn white_stays_near_white() {
        let white = Color::new(255, 255, 255);
        for cvd in CvdType::all() {
            let result = simulate_cvd(&white, *cvd);
            // White should stay very close to white (matrices sum to ~1.0 per row)
            assert!(result.r >= 250, "white r={} under {cvd:?}", result.r);
            assert!(result.g >= 250, "white g={} under {cvd:?}", result.g);
            assert!(result.b >= 250, "white b={} under {cvd:?}", result.b);
        }
    }

    #[test]
    fn red_shifts_under_protanopia() {
        let red = Color::new(255, 0, 0);
        let result = simulate_cvd(&red, CvdType::Protanopia);
        // Under protanopia, pure red should shift — red component should decrease
        // and the result should look brownish/dark
        assert!(result.r < 200, "red should diminish under protanopia: got r={}", result.r);
    }

    #[test]
    fn simulate_theme_preserves_name() {
        let theme = crate::Theme {
            name: "Test Theme".into(),
            background: Color::new(0, 0, 0),
            foreground: Color::new(255, 255, 255),
            cursor: Color::new(255, 0, 0),
            selection_background: Color::new(50, 50, 50),
            selection_foreground: Color::new(200, 200, 200),
            ansi: crate::AnsiColors::from_array(std::array::from_fn(|i| {
                Color::new(i as u8 * 16, i as u8 * 16, i as u8 * 16)
            })),
        };

        let simulated = simulate_theme(&theme, CvdType::Deuteranopia);
        assert_eq!(simulated.name, "Test Theme");
    }
}
