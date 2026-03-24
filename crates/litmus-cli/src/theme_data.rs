use litmus_model::provider::load_themes_dir;
use litmus_model::{AnsiColors, Color, Theme, error::ThemeError};
use std::path::{Path, PathBuf};

pub fn load_theme(path: &Path) -> Result<Theme, ThemeError> {
    let input = std::fs::read_to_string(path)?;
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext {
        "conf" => litmus_model::kitty::parse_kitty_theme(&input),
        "yaml" | "yml" => litmus_model::base16::parse_base16_theme(&input),
        "toml" => litmus_model::toml_format::parse_toml_theme(&input),
        other => Err(ThemeError::MissingField(format!("unsupported extension: {other}"))),
    }
}

/// Find the bundled themes directory relative to the executable or workspace root.
fn find_themes_dir() -> Option<PathBuf> {
    // Try relative to the executable (for installed builds)
    if let Ok(exe) = std::env::current_exe() {
        // exe is in target/debug or target/release — themes is at workspace root
        for ancestor in exe.ancestors().skip(1) {
            let candidate = ancestor.join("themes");
            if candidate.is_dir() {
                return Some(candidate);
            }
        }
    }
    // Try current working directory
    let cwd = std::env::current_dir().ok()?;
    for ancestor in cwd.ancestors() {
        let candidate = ancestor.join("themes");
        if candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}

/// Load bundled themes using the new ThemeDefinition + ProviderColors format.
///
/// For each ThemeDefinition, picks one ProviderColors (filtered by `provider` if given,
/// otherwise the first available) and converts to a `Theme` for rendering.
pub fn load_bundled_themes(provider: Option<&str>) -> Vec<Theme> {
    let Some(themes_dir) = find_themes_dir() else {
        return all_themes();
    };

    let (definitions, provider_colors) = match load_themes_dir(&themes_dir) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Warning: could not load themes from {}: {e}", themes_dir.display());
            return all_themes();
        }
    };

    let mut themes: Vec<Theme> = Vec::new();

    for def in &definitions {
        // Try the requested provider first, then fall back to any available
        let colors = if let Some(prov) = provider {
            provider_colors.get(&(def.slug.clone(), prov.to_string()))
        } else {
            // Pick first available provider (sorted for determinism)
            let mut providers: Vec<&String> = def.providers.keys().collect();
            providers.sort();
            providers
                .into_iter()
                .find_map(|p| provider_colors.get(&(def.slug.clone(), p.clone())))
        };

        if let Some(colors) = colors {
            themes.push(colors.to_theme(&def.name));
        }
    }

    if themes.is_empty() {
        return all_themes();
    }

    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}

pub fn tokyo_night() -> Theme {
    Theme {
        name: "Tokyo Night".to_string(),
        background: Color::new(0x1a, 0x1b, 0x26),
        foreground: Color::new(0xa9, 0xb1, 0xd6),
        cursor: Color::new(0xc0, 0xca, 0xf5),
        selection_background: Color::new(0x28, 0x3d, 0x5c),
        selection_foreground: Color::new(0xa9, 0xb1, 0xd6),
        ansi: AnsiColors::from_array([
            Color::new(0x15, 0x16, 0x22), // black
            Color::new(0xf7, 0x76, 0x8e), // red
            Color::new(0x9e, 0xce, 0x6a), // green
            Color::new(0xe0, 0xaf, 0x68), // yellow
            Color::new(0x7a, 0xa2, 0xf7), // blue
            Color::new(0xbb, 0x9a, 0xf7), // magenta
            Color::new(0x7d, 0xcf, 0xff), // cyan
            Color::new(0xa9, 0xb1, 0xd6), // white
            Color::new(0x41, 0x44, 0x68), // bright black
            Color::new(0xf7, 0x76, 0x8e), // bright red
            Color::new(0x9e, 0xce, 0x6a), // bright green
            Color::new(0xe0, 0xaf, 0x68), // bright yellow
            Color::new(0x7a, 0xa2, 0xf7), // bright blue
            Color::new(0xbb, 0x9a, 0xf7), // bright magenta
            Color::new(0x7d, 0xcf, 0xff), // bright cyan
            Color::new(0xc0, 0xca, 0xf5), // bright white
        ]),
    }
}

pub fn catppuccin_mocha() -> Theme {
    Theme {
        name: "Catppuccin Mocha".to_string(),
        background: Color::new(0x1e, 0x1e, 0x2e),
        foreground: Color::new(0xcd, 0xd6, 0xf4),
        cursor: Color::new(0xf5, 0xc2, 0xe7),
        selection_background: Color::new(0x31, 0x32, 0x44),
        selection_foreground: Color::new(0xcd, 0xd6, 0xf4),
        ansi: AnsiColors::from_array([
            Color::new(0x45, 0x47, 0x5a), // black
            Color::new(0xf3, 0x8b, 0xa8), // red
            Color::new(0xa6, 0xe3, 0xa1), // green
            Color::new(0xf9, 0xe2, 0xaf), // yellow
            Color::new(0x89, 0xb4, 0xfa), // blue
            Color::new(0xcb, 0xa6, 0xf7), // magenta
            Color::new(0x89, 0xdc, 0xeb), // cyan
            Color::new(0xba, 0xc2, 0xde), // white
            Color::new(0x58, 0x5b, 0x70), // bright black
            Color::new(0xf3, 0x8b, 0xa8), // bright red
            Color::new(0xa6, 0xe3, 0xa1), // bright green
            Color::new(0xf9, 0xe2, 0xaf), // bright yellow
            Color::new(0x89, 0xb4, 0xfa), // bright blue
            Color::new(0xcb, 0xa6, 0xf7), // bright magenta
            Color::new(0x89, 0xdc, 0xeb), // bright cyan
            Color::new(0xcd, 0xd6, 0xf4), // bright white
        ]),
    }
}

pub fn solarized_dark() -> Theme {
    Theme {
        name: "Solarized Dark".to_string(),
        background: Color::new(0x00, 0x2b, 0x36),
        foreground: Color::new(0x83, 0x94, 0x96),
        cursor: Color::new(0x83, 0x94, 0x96),
        selection_background: Color::new(0x07, 0x36, 0x42),
        selection_foreground: Color::new(0x83, 0x94, 0x96),
        ansi: AnsiColors::from_array([
            Color::new(0x07, 0x36, 0x42), // black
            Color::new(0xdc, 0x32, 0x2f), // red
            Color::new(0x85, 0x99, 0x00), // green
            Color::new(0xb5, 0x89, 0x00), // yellow
            Color::new(0x26, 0x8b, 0xd2), // blue
            Color::new(0xd3, 0x36, 0x82), // magenta
            Color::new(0x2a, 0xa1, 0x98), // cyan
            Color::new(0xee, 0xe8, 0xd5), // white
            Color::new(0x00, 0x2b, 0x36), // bright black
            Color::new(0xcb, 0x4b, 0x16), // bright red
            Color::new(0x58, 0x6e, 0x75), // bright green
            Color::new(0x65, 0x7b, 0x83), // bright yellow
            Color::new(0x83, 0x94, 0x96), // bright blue
            Color::new(0x6c, 0x71, 0xc4), // bright magenta
            Color::new(0x93, 0xa1, 0xa1), // bright cyan
            Color::new(0xfd, 0xf6, 0xe3), // bright white
        ]),
    }
}

pub fn all_themes() -> Vec<Theme> {
    vec![tokyo_night(), catppuccin_mocha(), solarized_dark()]
}
