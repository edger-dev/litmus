pub mod base16;
pub mod contrast;
pub mod cvd;
pub mod defaults;
pub mod error;
pub mod export;
pub mod kitty;
pub mod scene;
pub mod scenes;
pub mod toml_format;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn from_hex(s: &str) -> Option<Color> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color { r, g, b })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnsiColors {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
}

impl AnsiColors {
    pub fn from_array(c: [Color; 16]) -> Self {
        let [black, red, green, yellow, blue, magenta, cyan, white,
             bright_black, bright_red, bright_green, bright_yellow,
             bright_blue, bright_magenta, bright_cyan, bright_white] = c;
        Self {
            black, red, green, yellow, blue, magenta, cyan, white,
            bright_black, bright_red, bright_green, bright_yellow,
            bright_blue, bright_magenta, bright_cyan, bright_white,
        }
    }

    pub fn as_array(&self) -> [&Color; 16] {
        [
            &self.black, &self.red, &self.green, &self.yellow,
            &self.blue, &self.magenta, &self.cyan, &self.white,
            &self.bright_black, &self.bright_red, &self.bright_green, &self.bright_yellow,
            &self.bright_blue, &self.bright_magenta, &self.bright_cyan, &self.bright_white,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub selection_background: Color,
    pub selection_foreground: Color,
    pub ansi: AnsiColors,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ansi_colors_round_trip() {
        let colors: [Color; 16] = std::array::from_fn(|i| Color::new(i as u8, i as u8, i as u8));
        let ansi = AnsiColors::from_array(colors.clone());
        let arr = ansi.as_array();
        for (i, c) in arr.iter().enumerate() {
            assert_eq!(**c, colors[i]);
        }
    }
}
