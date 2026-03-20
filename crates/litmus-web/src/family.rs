use litmus_model::Theme;

/// Known theme family prefixes (checked in order, longest match wins).
static FAMILIES: &[&str] = &[
    "Catppuccin",
    "Everforest",
    "Gruvbox",
    "Rose Pine",
    "Rosé Pine",
    "Solarized",
    "Tokyo Night",
];

/// Extract the family name from a theme name.
/// Returns the matched family prefix, or the full name for standalone themes.
pub fn theme_family(name: &str) -> &str {
    for &family in FAMILIES {
        if name.starts_with(family) {
            return family;
        }
    }
    // Standalone theme — family is the theme itself
    name
}

/// A group of themes belonging to the same family.
pub struct ThemeFamily {
    pub name: String,
    pub themes: Vec<Theme>,
}

/// Group themes by family, preserving sort order within each family.
pub fn group_by_family(themes: &[Theme]) -> Vec<ThemeFamily> {
    let mut families: Vec<ThemeFamily> = Vec::new();

    for theme in themes {
        let family_name = theme_family(&theme.name).to_owned();
        if let Some(family) = families.iter_mut().find(|f| f.name == family_name) {
            family.themes.push(theme.clone());
        } else {
            families.push(ThemeFamily {
                name: family_name,
                themes: vec![theme.clone()],
            });
        }
    }

    families
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_families() {
        assert_eq!(theme_family("Catppuccin Mocha"), "Catppuccin");
        assert_eq!(theme_family("Catppuccin Frappe"), "Catppuccin");
        assert_eq!(theme_family("Tokyo Night"), "Tokyo Night");
        assert_eq!(theme_family("Tokyo Night Storm"), "Tokyo Night");
        assert_eq!(theme_family("Rose Pine Dawn"), "Rose Pine");
    }

    #[test]
    fn standalone_themes() {
        assert_eq!(theme_family("Dracula"), "Dracula");
        assert_eq!(theme_family("Nord"), "Nord");
        assert_eq!(theme_family("Kanagawa"), "Kanagawa");
    }
}
