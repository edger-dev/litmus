use litmus_model::Theme;
use std::path::Path;

mod kitty;
mod wezterm;
pub use kitty::KittyProvider;
pub use wezterm::WeztermProvider;

/// Terminal geometry for screenshot capture.
#[derive(Debug, Clone)]
pub struct TermGeometry {
    /// Terminal columns (character width)
    pub cols: u32,
    /// Terminal rows (character height)
    pub rows: u32,
    /// Font size in points
    pub font_size: f32,
    /// Font family name
    pub font_family: String,
}

impl Default for TermGeometry {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            font_size: 12.0,
            font_family: "FiraCode".to_string(),
        }
    }
}

/// Trait implemented by each supported terminal emulator provider.
pub trait ProviderCapture: Send + Sync {
    /// Short machine-readable identifier, e.g. "kitty".
    fn slug(&self) -> &str;

    /// Human-readable name, e.g. "Kitty".
    fn name(&self) -> &str;

    /// Generate the provider's config file content for the given theme and geometry.
    fn generate_config(&self, theme: &Theme, geometry: &TermGeometry) -> String;

    /// Returns the config file extension, e.g. "conf".
    fn config_extension(&self) -> &str;

    /// Build the command arguments to launch this terminal.
    /// - `config_path`: path to the generated config file
    /// - `command`: the shell command to run inside the terminal
    ///   (should not exit immediately; the capture tool manages lifecycle)
    fn build_launch_args(&self, config_path: &Path, command: &str) -> Vec<String>;
}

/// Returns all registered providers.
pub fn all_providers() -> Vec<Box<dyn ProviderCapture>> {
    vec![
        Box::new(KittyProvider),
        Box::new(WeztermProvider),
    ]
}

/// Find a provider by slug.
pub fn find_provider(slug: &str) -> Option<Box<dyn ProviderCapture>> {
    all_providers().into_iter().find(|p| p.slug() == slug)
}
