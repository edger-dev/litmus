use std::collections::HashMap;
use std::sync::OnceLock;

use litmus_model::provider::{
    parse_provider_colors, parse_theme_definition, ProviderColors, ProviderColorsKey,
    ThemeDefinition,
};
use litmus_model::Theme;

/// Cached parsed theme data (definitions + provider colors).
struct ThemeDataCache {
    definitions: Vec<ThemeDefinition>,
    colors: HashMap<ProviderColorsKey, ProviderColors>,
    providers: Vec<String>,
}

static THEME_DATA_CACHE: OnceLock<ThemeDataCache> = OnceLock::new();

fn cached_data() -> &'static ThemeDataCache {
    THEME_DATA_CACHE.get_or_init(|| {
        let (definitions, colors) = parse_all_theme_data();
        let mut providers: Vec<String> = colors
            .keys()
            .map(|(_, p)| p.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        providers.sort();
        ThemeDataCache {
            definitions,
            colors,
            providers,
        }
    })
}

/// Embedded theme definitions: (slug, toml_content).
static DEFINITION_DATA: &[(&str, &str)] = &[
    ("andromeda", include_str!("../../../themes/andromeda.toml")),
    ("ayu-dark", include_str!("../../../themes/ayu/ayu-dark.toml")),
    ("ayu-light", include_str!("../../../themes/ayu/ayu-light.toml")),
    ("frappe", include_str!("../../../themes/catppuccin/frappe.toml")),
    ("latte", include_str!("../../../themes/catppuccin/latte.toml")),
    ("macchiato", include_str!("../../../themes/catppuccin/macchiato.toml")),
    ("mocha", include_str!("../../../themes/catppuccin/mocha.toml")),
    ("cyberdream-dark", include_str!("../../../themes/cyberdream/cyberdream-dark.toml")),
    ("cyberdream-light", include_str!("../../../themes/cyberdream/cyberdream-light.toml")),
    ("dracula", include_str!("../../../themes/dracula.toml")),
    ("everforest-dark", include_str!("../../../themes/everforest/everforest-dark.toml")),
    ("everforest-light", include_str!("../../../themes/everforest/everforest-light.toml")),
    ("flexoki-dark", include_str!("../../../themes/flexoki/flexoki-dark.toml")),
    ("flexoki-light", include_str!("../../../themes/flexoki/flexoki-light.toml")),
    ("github-dark-dimmed", include_str!("../../../themes/github/github-dark-dimmed.toml")),
    ("github-dark", include_str!("../../../themes/github/github-dark.toml")),
    ("github-light", include_str!("../../../themes/github/github-light.toml")),
    ("gruvbox-dark", include_str!("../../../themes/gruvbox/gruvbox-dark.toml")),
    ("gruvbox-light", include_str!("../../../themes/gruvbox/gruvbox-light.toml")),
    ("horizon", include_str!("../../../themes/horizon.toml")),
    ("iceberg-dark", include_str!("../../../themes/iceberg/iceberg-dark.toml")),
    ("iceberg-light", include_str!("../../../themes/iceberg/iceberg-light.toml")),
    ("kanagawa-dragon", include_str!("../../../themes/kanagawa/kanagawa-dragon.toml")),
    ("kanagawa-wave", include_str!("../../../themes/kanagawa/kanagawa-wave.toml")),
    ("material", include_str!("../../../themes/material.toml")),
    ("melange-dark", include_str!("../../../themes/melange/melange-dark.toml")),
    ("melange-light", include_str!("../../../themes/melange/melange-light.toml")),
    ("modus-operandi", include_str!("../../../themes/modus/modus-operandi.toml")),
    ("modus-vivendi", include_str!("../../../themes/modus/modus-vivendi.toml")),
    ("monokai", include_str!("../../../themes/monokai.toml")),
    ("moonlight", include_str!("../../../themes/moonlight.toml")),
    ("dawnfox", include_str!("../../../themes/nightfox/dawnfox.toml")),
    ("dayfox", include_str!("../../../themes/nightfox/dayfox.toml")),
    ("duskfox", include_str!("../../../themes/nightfox/duskfox.toml")),
    ("nightfox", include_str!("../../../themes/nightfox/nightfox.toml")),
    ("nordfox", include_str!("../../../themes/nightfox/nordfox.toml")),
    ("terafox", include_str!("../../../themes/nightfox/terafox.toml")),
    ("light-owl", include_str!("../../../themes/night-owl/light-owl.toml")),
    ("night-owl", include_str!("../../../themes/night-owl/night-owl.toml")),
    ("nord", include_str!("../../../themes/nord.toml")),
    ("one-dark", include_str!("../../../themes/one-dark.toml")),
    ("one-light", include_str!("../../../themes/one-light.toml")),
    ("oxocarbon-dark", include_str!("../../../themes/oxocarbon/oxocarbon-dark.toml")),
    ("oxocarbon-light", include_str!("../../../themes/oxocarbon/oxocarbon-light.toml")),
    ("palenight", include_str!("../../../themes/palenight.toml")),
    ("poimandres", include_str!("../../../themes/poimandres.toml")),
    ("rose-pine-dawn", include_str!("../../../themes/rose-pine/rose-pine-dawn.toml")),
    ("rose-pine-moon", include_str!("../../../themes/rose-pine/rose-pine-moon.toml")),
    ("rose-pine", include_str!("../../../themes/rose-pine/rose-pine.toml")),
    ("snazzy", include_str!("../../../themes/snazzy.toml")),
    ("solarized-dark", include_str!("../../../themes/solarized/solarized-dark.toml")),
    ("solarized-light", include_str!("../../../themes/solarized/solarized-light.toml")),
    ("sonokai-shusia", include_str!("../../../themes/sonokai/sonokai-shusia.toml")),
    ("sonokai", include_str!("../../../themes/sonokai/sonokai.toml")),
    ("tender", include_str!("../../../themes/tender.toml")),
    ("tokyo-night-day", include_str!("../../../themes/tokyo-night/tokyo-night-day.toml")),
    ("tokyo-night-storm", include_str!("../../../themes/tokyo-night/tokyo-night-storm.toml")),
    ("tokyo-night", include_str!("../../../themes/tokyo-night/tokyo-night.toml")),
    ("vesper", include_str!("../../../themes/vesper.toml")),
    ("zenburn", include_str!("../../../themes/zenburn.toml")),
];

/// Embedded provider color data: (theme_slug, toml_content).
static PROVIDER_COLORS_DATA: &[(&str, &str)] = &[
    ("andromeda", include_str!("../../../themes/andromeda.wezterm.toml")),
    ("ayu-dark", include_str!("../../../themes/ayu/ayu-dark.kitty.toml")),
    ("ayu-dark", include_str!("../../../themes/ayu/ayu-dark.wezterm.toml")),
    ("ayu-light", include_str!("../../../themes/ayu/ayu-light.kitty.toml")),
    ("frappe", include_str!("../../../themes/catppuccin/frappe.kitty.toml")),
    ("frappe", include_str!("../../../themes/catppuccin/frappe.wezterm.toml")),
    ("latte", include_str!("../../../themes/catppuccin/latte.kitty.toml")),
    ("latte", include_str!("../../../themes/catppuccin/latte.wezterm.toml")),
    ("macchiato", include_str!("../../../themes/catppuccin/macchiato.kitty.toml")),
    ("macchiato", include_str!("../../../themes/catppuccin/macchiato.wezterm.toml")),
    ("mocha", include_str!("../../../themes/catppuccin/mocha.kitty.toml")),
    ("mocha", include_str!("../../../themes/catppuccin/mocha.wezterm.toml")),
    ("cyberdream-dark", include_str!("../../../themes/cyberdream/cyberdream-dark.kitty.toml")),
    ("cyberdream-dark", include_str!("../../../themes/cyberdream/cyberdream-dark.wezterm.toml")),
    ("cyberdream-light", include_str!("../../../themes/cyberdream/cyberdream-light.kitty.toml")),
    ("cyberdream-light", include_str!("../../../themes/cyberdream/cyberdream-light.wezterm.toml")),
    ("dracula", include_str!("../../../themes/dracula.kitty.toml")),
    ("dracula", include_str!("../../../themes/dracula.wezterm.toml")),
    ("everforest-dark", include_str!("../../../themes/everforest/everforest-dark.kitty.toml")),
    ("everforest-dark", include_str!("../../../themes/everforest/everforest-dark.wezterm.toml")),
    ("everforest-light", include_str!("../../../themes/everforest/everforest-light.kitty.toml")),
    ("everforest-light", include_str!("../../../themes/everforest/everforest-light.wezterm.toml")),
    ("flexoki-dark", include_str!("../../../themes/flexoki/flexoki-dark.kitty.toml")),
    ("flexoki-dark", include_str!("../../../themes/flexoki/flexoki-dark.wezterm.toml")),
    ("flexoki-light", include_str!("../../../themes/flexoki/flexoki-light.kitty.toml")),
    ("flexoki-light", include_str!("../../../themes/flexoki/flexoki-light.wezterm.toml")),
    ("github-dark-dimmed", include_str!("../../../themes/github/github-dark-dimmed.kitty.toml")),
    ("github-dark", include_str!("../../../themes/github/github-dark.kitty.toml")),
    ("github-dark", include_str!("../../../themes/github/github-dark.wezterm.toml")),
    ("github-light", include_str!("../../../themes/github/github-light.kitty.toml")),
    ("gruvbox-dark", include_str!("../../../themes/gruvbox/gruvbox-dark.kitty.toml")),
    ("gruvbox-dark", include_str!("../../../themes/gruvbox/gruvbox-dark.wezterm.toml")),
    ("gruvbox-light", include_str!("../../../themes/gruvbox/gruvbox-light.kitty.toml")),
    ("gruvbox-light", include_str!("../../../themes/gruvbox/gruvbox-light.wezterm.toml")),
    ("horizon", include_str!("../../../themes/horizon.wezterm.toml")),
    ("iceberg-dark", include_str!("../../../themes/iceberg/iceberg-dark.wezterm.toml")),
    ("iceberg-light", include_str!("../../../themes/iceberg/iceberg-light.wezterm.toml")),
    ("kanagawa-dragon", include_str!("../../../themes/kanagawa/kanagawa-dragon.kitty.toml")),
    ("kanagawa-dragon", include_str!("../../../themes/kanagawa/kanagawa-dragon.wezterm.toml")),
    ("kanagawa-wave", include_str!("../../../themes/kanagawa/kanagawa-wave.kitty.toml")),
    ("material", include_str!("../../../themes/material.kitty.toml")),
    ("material", include_str!("../../../themes/material.wezterm.toml")),
    ("melange-dark", include_str!("../../../themes/melange/melange-dark.kitty.toml")),
    ("melange-dark", include_str!("../../../themes/melange/melange-dark.wezterm.toml")),
    ("melange-light", include_str!("../../../themes/melange/melange-light.kitty.toml")),
    ("melange-light", include_str!("../../../themes/melange/melange-light.wezterm.toml")),
    ("modus-operandi", include_str!("../../../themes/modus/modus-operandi.kitty.toml")),
    ("modus-vivendi", include_str!("../../../themes/modus/modus-vivendi.kitty.toml")),
    ("monokai", include_str!("../../../themes/monokai.kitty.toml")),
    ("monokai", include_str!("../../../themes/monokai.wezterm.toml")),
    ("moonlight", include_str!("../../../themes/moonlight.kitty.toml")),
    ("dawnfox", include_str!("../../../themes/nightfox/dawnfox.kitty.toml")),
    ("dawnfox", include_str!("../../../themes/nightfox/dawnfox.wezterm.toml")),
    ("dayfox", include_str!("../../../themes/nightfox/dayfox.kitty.toml")),
    ("dayfox", include_str!("../../../themes/nightfox/dayfox.wezterm.toml")),
    ("duskfox", include_str!("../../../themes/nightfox/duskfox.kitty.toml")),
    ("duskfox", include_str!("../../../themes/nightfox/duskfox.wezterm.toml")),
    ("nightfox", include_str!("../../../themes/nightfox/nightfox.kitty.toml")),
    ("nightfox", include_str!("../../../themes/nightfox/nightfox.wezterm.toml")),
    ("nordfox", include_str!("../../../themes/nightfox/nordfox.kitty.toml")),
    ("nordfox", include_str!("../../../themes/nightfox/nordfox.wezterm.toml")),
    ("terafox", include_str!("../../../themes/nightfox/terafox.kitty.toml")),
    ("terafox", include_str!("../../../themes/nightfox/terafox.wezterm.toml")),
    ("light-owl", include_str!("../../../themes/night-owl/light-owl.kitty.toml")),
    ("light-owl", include_str!("../../../themes/night-owl/light-owl.wezterm.toml")),
    ("night-owl", include_str!("../../../themes/night-owl/night-owl.kitty.toml")),
    ("night-owl", include_str!("../../../themes/night-owl/night-owl.wezterm.toml")),
    ("nord", include_str!("../../../themes/nord.kitty.toml")),
    ("nord", include_str!("../../../themes/nord.wezterm.toml")),
    ("one-dark", include_str!("../../../themes/one-dark.kitty.toml")),
    ("one-dark", include_str!("../../../themes/one-dark.wezterm.toml")),
    ("one-light", include_str!("../../../themes/one-light.wezterm.toml")),
    ("oxocarbon-dark", include_str!("../../../themes/oxocarbon/oxocarbon-dark.wezterm.toml")),
    ("oxocarbon-light", include_str!("../../../themes/oxocarbon/oxocarbon-light.kitty.toml")),
    ("oxocarbon-light", include_str!("../../../themes/oxocarbon/oxocarbon-light.wezterm.toml")),
    ("palenight", include_str!("../../../themes/palenight.wezterm.toml")),
    ("poimandres", include_str!("../../../themes/poimandres.wezterm.toml")),
    ("rose-pine-dawn", include_str!("../../../themes/rose-pine/rose-pine-dawn.kitty.toml")),
    ("rose-pine", include_str!("../../../themes/rose-pine/rose-pine.kitty.toml")),
    ("rose-pine-moon", include_str!("../../../themes/rose-pine/rose-pine-moon.kitty.toml")),
    ("snazzy", include_str!("../../../themes/snazzy.kitty.toml")),
    ("snazzy", include_str!("../../../themes/snazzy.wezterm.toml")),
    ("solarized-dark", include_str!("../../../themes/solarized/solarized-dark.kitty.toml")),
    ("solarized-dark", include_str!("../../../themes/solarized/solarized-dark.wezterm.toml")),
    ("solarized-light", include_str!("../../../themes/solarized/solarized-light.kitty.toml")),
    ("solarized-light", include_str!("../../../themes/solarized/solarized-light.wezterm.toml")),
    ("sonokai-shusia", include_str!("../../../themes/sonokai/sonokai-shusia.wezterm.toml")),
    ("sonokai", include_str!("../../../themes/sonokai/sonokai.wezterm.toml")),
    ("tender", include_str!("../../../themes/tender.kitty.toml")),
    ("tender", include_str!("../../../themes/tender.wezterm.toml")),
    ("tokyo-night-day", include_str!("../../../themes/tokyo-night/tokyo-night-day.kitty.toml")),
    ("tokyo-night-day", include_str!("../../../themes/tokyo-night/tokyo-night-day.wezterm.toml")),
    ("tokyo-night", include_str!("../../../themes/tokyo-night/tokyo-night.kitty.toml")),
    ("tokyo-night-storm", include_str!("../../../themes/tokyo-night/tokyo-night-storm.kitty.toml")),
    ("tokyo-night-storm", include_str!("../../../themes/tokyo-night/tokyo-night-storm.wezterm.toml")),
    ("tokyo-night", include_str!("../../../themes/tokyo-night/tokyo-night.wezterm.toml")),
    ("vesper", include_str!("../../../themes/vesper.wezterm.toml")),
    ("zenburn", include_str!("../../../themes/zenburn.kitty.toml")),
    ("zenburn", include_str!("../../../themes/zenburn.wezterm.toml")),
];

/// Parse all embedded theme data (uncached — use `cached_data()` for the cached version).
fn parse_all_theme_data() -> (Vec<ThemeDefinition>, HashMap<ProviderColorsKey, ProviderColors>) {
    let mut definitions = Vec::new();
    for (slug, toml_str) in DEFINITION_DATA {
        match parse_theme_definition(toml_str, slug) {
            Ok(def) => definitions.push(def),
            Err(e) => eprintln!("Warning: failed to parse definition '{slug}': {e}"),
        }
    }

    let mut colors: HashMap<ProviderColorsKey, ProviderColors> = HashMap::new();
    for (theme_slug, toml_str) in PROVIDER_COLORS_DATA {
        match parse_provider_colors(toml_str) {
            Ok(pc) => {
                colors.insert(
                    (theme_slug.to_string(), pc.provider.clone()),
                    pc,
                );
            }
            Err(e) => eprintln!("Warning: failed to parse provider colors for '{theme_slug}': {e}"),
        }
    }

    (definitions, colors)
}

/// Parsed theme data: definitions and provider color map (cached).
#[cfg(test)]
pub fn load_embedded_theme_data() -> (&'static Vec<ThemeDefinition>, &'static HashMap<ProviderColorsKey, ProviderColors>) {
    let cache = cached_data();
    (&cache.definitions, &cache.colors)
}

/// Sorted list of all available provider slugs (cached).
pub fn available_providers() -> &'static Vec<String> {
    &cached_data().providers
}

/// Build renderable themes for a specific provider. Returns themes sorted by name.
/// Only includes definitions that have provider colors for the requested provider.
pub fn themes_for_provider(provider: &str) -> Vec<Theme> {
    let cache = cached_data();
    let mut themes: Vec<Theme> = cache
        .definitions
        .iter()
        .filter_map(|def| {
            let key = (def.slug.clone(), provider.to_string());
            cache.colors.get(&key).map(|pc| pc.to_theme(&def.name))
        })
        .collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}

/// Build all themes with availability flag for the given provider.
/// Returns every definition as a renderable Theme (using the requested provider's colors
/// if available, otherwise falling back to the first available provider).
/// The bool indicates whether the theme is natively available for the requested provider.
/// Sorted alphabetically by name.
pub fn all_themes_with_availability(provider: &str) -> Vec<(Theme, bool)> {
    let cache = cached_data();
    let mut results: Vec<(Theme, bool)> = cache
        .definitions
        .iter()
        .filter_map(|def| {
            let key = (def.slug.clone(), provider.to_string());
            if let Some(pc) = cache.colors.get(&key) {
                // Available: use this provider's colors
                Some((pc.to_theme(&def.name), true))
            } else {
                // Unavailable: fall back to first available provider
                let mut providers: Vec<&String> = def.providers.keys().collect();
                providers.sort();
                providers
                    .into_iter()
                    .find_map(|p| cache.colors.get(&(def.slug.clone(), p.clone())))
                    .map(|pc| (pc.to_theme(&def.name), false))
            }
        })
        .collect();
    results.sort_by(|a, b| a.0.name.to_lowercase().cmp(&b.0.name.to_lowercase()));
    results
}

/// Check whether a theme (by slug) is available for a given provider.
pub fn theme_available_for_provider(slug: &str, provider: &str) -> bool {
    let cache = cached_data();
    cache.colors.contains_key(&(slug.to_string(), provider.to_string()))
}

/// Load all embedded themes using the first available provider per theme.
/// Backward-compatible with the old API — used by tests.
#[cfg(test)]
pub fn load_embedded_themes() -> Vec<Theme> {
    let cache = cached_data();
    let mut themes: Vec<Theme> = cache
        .definitions
        .iter()
        .filter_map(|def| {
            let mut providers: Vec<&String> = def.providers.keys().collect();
            providers.sort();
            providers
                .into_iter()
                .find_map(|p| cache.colors.get(&(def.slug.clone(), p.clone())))
                .map(|pc| pc.to_theme(&def.name))
        })
        .collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_embedded_theme_data_parses_all_definitions() {
        let (defs, _) = load_embedded_theme_data();
        assert_eq!(defs.len(), DEFINITION_DATA.len());
        for def in defs {
            assert!(!def.name.is_empty());
            assert!(!def.slug.is_empty());
        }
    }

    #[test]
    fn load_embedded_theme_data_parses_all_provider_colors() {
        let (_, colors) = load_embedded_theme_data();
        assert_eq!(colors.len(), PROVIDER_COLORS_DATA.len());
    }

    #[test]
    fn available_providers_returns_kitty_and_wezterm() {
        let providers = available_providers();
        assert_eq!(providers, &["kitty", "wezterm"]);
    }

    #[test]
    fn themes_for_provider_kitty_non_empty() {
        let themes = themes_for_provider("kitty");
        assert!(!themes.is_empty());
        // Should be sorted by name
        for w in themes.windows(2) {
            assert!(w[0].name <= w[1].name);
        }
    }

    #[test]
    fn themes_for_provider_wezterm_non_empty() {
        let themes = themes_for_provider("wezterm");
        assert!(!themes.is_empty());
    }

    #[test]
    fn themes_for_provider_nonexistent_empty() {
        let themes = themes_for_provider("alacritty");
        assert!(themes.is_empty());
    }

    #[test]
    fn every_definition_has_at_least_one_provider() {
        let (defs, colors) = load_embedded_theme_data();
        for def in defs {
            let has_colors = colors
                .keys()
                .any(|(slug, _)| slug == &def.slug);
            assert!(has_colors, "definition '{}' has no provider colors", def.slug);
        }
    }

    #[test]
    fn load_embedded_themes_backward_compat() {
        let themes = load_embedded_themes();
        assert!(!themes.is_empty());
        // Should be sorted by name
        for w in themes.windows(2) {
            assert!(w[0].name <= w[1].name);
        }
    }

    #[test]
    fn definition_slugs_are_unique() {
        let (defs, _) = load_embedded_theme_data();
        let mut slugs: Vec<&str> = defs.iter().map(|d| d.slug.as_str()).collect();
        slugs.sort();
        let len_before = slugs.len();
        slugs.dedup();
        assert_eq!(slugs.len(), len_before, "duplicate slugs found");
    }

    #[test]
    fn all_themes_with_availability_returns_all_definitions() {
        let all = all_themes_with_availability("kitty");
        assert_eq!(all.len(), DEFINITION_DATA.len(), "should return one entry per definition");
    }

    #[test]
    fn all_themes_with_availability_sorted_by_name() {
        let all = all_themes_with_availability("kitty");
        for w in all.windows(2) {
            assert!(
                w[0].0.name.to_lowercase() <= w[1].0.name.to_lowercase(),
                "not sorted: {} > {}",
                w[0].0.name,
                w[1].0.name,
            );
        }
    }

    #[test]
    fn all_themes_with_availability_marks_available_correctly() {
        let kitty_themes = all_themes_with_availability("kitty");
        let kitty_only = themes_for_provider("kitty");
        let available_count = kitty_themes.iter().filter(|(_, a)| *a).count();
        assert_eq!(available_count, kitty_only.len());
    }

    #[test]
    fn all_themes_with_availability_has_unavailable_themes() {
        // Kitty and wezterm don't cover all the same themes, so there should be some unavailable
        let kitty = all_themes_with_availability("kitty");
        let unavailable = kitty.iter().filter(|(_, a)| !*a).count();
        assert!(unavailable > 0, "expected some unavailable themes for kitty");
    }

    #[test]
    fn all_themes_with_availability_nonexistent_provider_all_unavailable() {
        let all = all_themes_with_availability("alacritty");
        assert!(all.iter().all(|(_, a)| !*a), "all should be unavailable for nonexistent provider");
        assert_eq!(all.len(), DEFINITION_DATA.len());
    }
}
