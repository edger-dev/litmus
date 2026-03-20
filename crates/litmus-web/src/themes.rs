use litmus_model::Theme;

/// Embedded theme data for WASM builds.
static THEME_DATA: &[&str] = &[
    include_str!("../../../themes/catppuccin/frappe.toml"),
    include_str!("../../../themes/catppuccin/latte.toml"),
    include_str!("../../../themes/catppuccin/macchiato.toml"),
    include_str!("../../../themes/catppuccin/mocha.toml"),
    include_str!("../../../themes/dracula.toml"),
    include_str!("../../../themes/everforest/everforest-dark.toml"),
    include_str!("../../../themes/everforest/everforest-light.toml"),
    include_str!("../../../themes/gruvbox/gruvbox-dark.toml"),
    include_str!("../../../themes/gruvbox/gruvbox-light.toml"),
    include_str!("../../../themes/kanagawa.toml"),
    include_str!("../../../themes/nord.toml"),
    include_str!("../../../themes/rose-pine/rose-pine.toml"),
    include_str!("../../../themes/rose-pine/rose-pine-dawn.toml"),
    include_str!("../../../themes/rose-pine/rose-pine-moon.toml"),
    include_str!("../../../themes/solarized/solarized-dark.toml"),
    include_str!("../../../themes/solarized/solarized-light.toml"),
    include_str!("../../../themes/tokyo-night/tokyo-night.toml"),
    include_str!("../../../themes/tokyo-night/tokyo-night-day.toml"),
    include_str!("../../../themes/tokyo-night/tokyo-night-storm.toml"),
];

/// Load all embedded themes. Returns themes sorted by name.
pub fn load_embedded_themes() -> Vec<Theme> {
    let mut themes: Vec<Theme> = THEME_DATA
        .iter()
        .filter_map(|toml_str| litmus_model::toml_format::parse_toml_theme(toml_str).ok())
        .collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}
