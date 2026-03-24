use std::collections::HashMap;
use std::path::Path;

use anyhow::{bail, Context, Result};
use litmus_model::kitty::parse_kitty_theme;
use litmus_model::provider::{ProviderColors, ThemeDefinition, parse_theme_definition};
use litmus_model::wezterm::parse_wezterm_scheme;
use serde::Deserialize;

/// Index of kitty themes: maps theme name → .conf file path (relative to vendor dir).
pub fn build_kitty_index(vendor_dir: &Path) -> Result<HashMap<String, String>> {
    let json_path = vendor_dir.join("kitty-themes/themes.json");
    let content = std::fs::read_to_string(&json_path)
        .with_context(|| format!("read {}", json_path.display()))?;

    let entries: Vec<KittyThemeEntry> =
        serde_json::from_str(&content).context("parse kitty themes.json")?;

    let mut index = HashMap::new();
    for entry in entries {
        index.insert(entry.name, entry.file);
    }
    Ok(index)
}

#[derive(Deserialize)]
struct KittyThemeEntry {
    name: String,
    file: String,
}

/// Index of wezterm themes: maps theme name → .toml file path (relative to vendor dir).
pub fn build_wezterm_index(vendor_dir: &Path) -> Result<HashMap<String, String>> {
    let schemes_dir = vendor_dir.join("wezterm-colorschemes/schemes");
    let mut index = HashMap::new();

    for entry in std::fs::read_dir(&schemes_dir)
        .with_context(|| format!("read {}", schemes_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("read {}", path.display()))?;

        // Extract name from [metadata] section without full parse
        if let Some(name) = extract_wezterm_name(&content) {
            let rel_path = format!(
                "wezterm-colorschemes/schemes/{}",
                path.file_name().unwrap().to_str().unwrap()
            );
            index.insert(name.clone(), rel_path.clone());

            // Also index aliases
            if let Some(aliases) = extract_wezterm_aliases(&content) {
                for alias in aliases {
                    index.insert(alias, rel_path.clone());
                }
            }
        }
    }

    Ok(index)
}

/// Quick extraction of name from wezterm TOML without full deserialization.
fn extract_wezterm_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("name = ") {
            return Some(rest.trim_matches('"').to_string());
        }
    }
    None
}

/// Quick extraction of aliases from wezterm TOML.
fn extract_wezterm_aliases(content: &str) -> Option<Vec<String>> {
    // Simple approach: find aliases = [...] line(s)
    let mut in_aliases = false;
    let mut aliases = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("aliases = [") {
            in_aliases = true;
            // Check for single-line array like: aliases = ["Foo", "Bar"]
            if let Some(rest) = trimmed.strip_prefix("aliases = [")
                && let Some(inner) = rest.strip_suffix(']')
            {
                for item in inner.split(',') {
                    let item = item.trim().trim_matches('"');
                    if !item.is_empty() {
                        aliases.push(item.to_string());
                    }
                }
                return Some(aliases);
            }
            continue;
        }
        if in_aliases {
            if trimmed == "]" {
                return Some(aliases);
            }
            let item = trimmed.trim_matches(|c: char| c == '"' || c == ',' || c.is_whitespace());
            if !item.is_empty() {
                aliases.push(item.to_string());
            }
        }
    }

    None
}

/// Scan a themes directory for ThemeDefinition files.
pub fn find_theme_definitions(themes_dir: &Path) -> Result<Vec<(ThemeDefinition, std::path::PathBuf)>> {
    let mut defs = Vec::new();

    for entry in walkdir::WalkDir::new(themes_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };

        // Only match ThemeDefinition files (no dots in stem before .toml)
        if !file_name.ends_with(".toml") {
            continue;
        }
        let stem = &file_name[..file_name.len() - 5];
        if stem.contains('.') {
            // This is a provider colors file, skip
            continue;
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("read {}", path.display()))?;

        // Try to parse as ThemeDefinition — skip if it doesn't match the format
        // (e.g., old-format theme files that have [colors] instead of [providers])
        match parse_theme_definition(&content, stem) {
            Ok(def) if !def.providers.is_empty() => {
                defs.push((def, path.parent().unwrap().to_path_buf()));
            }
            _ => continue,
        }
    }

    defs.sort_by(|a, b| a.0.slug.cmp(&b.0.slug));
    Ok(defs)
}

/// Extract colors for a single provider mapping.
pub fn extract_provider_colors(
    vendor_dir: &Path,
    provider_slug: &str,
    provider_theme_name: &str,
    kitty_index: &HashMap<String, String>,
    wezterm_index: &HashMap<String, String>,
) -> Result<ProviderColors> {
    match provider_slug {
        "kitty" => {
            let rel_path = kitty_index
                .get(provider_theme_name)
                .with_context(|| format!("kitty theme '{}' not found in vendor index", provider_theme_name))?;
            let conf_path = vendor_dir.join("kitty-themes").join(rel_path);
            let content = std::fs::read_to_string(&conf_path)
                .with_context(|| format!("read {}", conf_path.display()))?;
            let theme = parse_kitty_theme(&content)
                .with_context(|| format!("parse kitty theme '{}'", provider_theme_name))?;
            Ok(ProviderColors::from_theme(&theme, "kitty", "vendored"))
        }
        "wezterm" => {
            let rel_path = wezterm_index
                .get(provider_theme_name)
                .with_context(|| format!("wezterm theme '{}' not found in vendor index", provider_theme_name))?;
            let toml_path = vendor_dir.join(rel_path);
            let content = std::fs::read_to_string(&toml_path)
                .with_context(|| format!("read {}", toml_path.display()))?;
            let theme = parse_wezterm_scheme(&content)
                .with_context(|| format!("parse wezterm theme '{}'", provider_theme_name))?;
            Ok(ProviderColors::from_theme(&theme, "wezterm", "vendored"))
        }
        _ => bail!("unknown provider '{}'; supported: kitty, wezterm", provider_slug),
    }
}
