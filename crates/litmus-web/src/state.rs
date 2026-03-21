use litmus_model::cvd::CvdType;

pub const MAX_COMPARE: usize = 4;
pub const MAX_SHORTLIST: usize = 5;

/// Global shortlist state — stores slugs of themes the user has favorited.
#[derive(Clone, Default)]
pub struct Shortlist(pub Vec<String>);

/// Filter mode for light/dark themes.
#[derive(Clone, Copy, PartialEq)]
pub enum VariantFilter {
    All,
    Dark,
    Light,
}

/// Filter state — used locally on the browse page.
#[derive(Clone)]
pub struct FilterState {
    pub query: String,
    pub variant: VariantFilter,
    pub min_readability: Option<u8>,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            query: String::new(),
            variant: VariantFilter::All,
            min_readability: None,
        }
    }
}

/// Global CVD simulation state — affects all pages.
#[derive(Clone, Default)]
pub struct CvdSimulation(pub Option<CvdType>);

/// Currently selected scene (by index) — used on ThemeDetail.
#[derive(Clone, Default)]
pub struct ActiveScene(pub Option<usize>);

/// App chrome theme slug.
#[derive(Clone)]
pub struct AppThemeSlug(pub Option<String>);

impl Default for AppThemeSlug {
    fn default() -> Self {
        Self(Some("tokyo-night".to_string()))
    }
}

/// Mobile sidebar drawer state.
#[derive(Clone, Default)]
pub struct SidebarOpen(pub bool);

pub fn is_light_theme(theme: &litmus_model::Theme) -> bool {
    litmus_model::contrast::relative_luminance(&theme.background) > 0.5
}

pub fn theme_passes_filter(
    theme: &litmus_model::Theme,
    filter: &FilterState,
) -> bool {
    if !filter.query.is_empty() {
        let q = filter.query.to_lowercase();
        let name = theme.name.to_lowercase();
        let fam = crate::family::theme_family(&theme.name).to_lowercase();
        if !name.contains(&q) && !fam.contains(&q) {
            return false;
        }
    }
    match filter.variant {
        VariantFilter::All => {}
        VariantFilter::Dark => {
            if is_light_theme(theme) {
                return false;
            }
        }
        VariantFilter::Light => {
            if !is_light_theme(theme) {
                return false;
            }
        }
    }
    if let Some(min) = filter.min_readability {
        let score = litmus_model::contrast::readability_score(theme);
        if score < min as f64 {
            return false;
        }
    }
    true
}

/// Optionally apply CVD simulation to a theme.
pub fn maybe_simulate(theme: &litmus_model::Theme, cvd: Option<CvdType>) -> litmus_model::Theme {
    match cvd {
        Some(ct) => litmus_model::cvd::simulate_theme(theme, ct),
        None => theme.clone(),
    }
}

pub fn theme_slug(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}
