use std::sync::OnceLock;

use litmus_model::term_output::TermOutput;

/// Embedded fixture output JSON data.
static FIXTURE_DATA: &[(&str, &str)] = &[
    ("git-diff", include_str!("../../../fixtures/git-diff/output.json")),
    ("git-log", include_str!("../../../fixtures/git-log/output.json")),
    ("ls-color", include_str!("../../../fixtures/ls-color/output.json")),
    ("cargo-build", include_str!("../../../fixtures/cargo-build/output.json")),
    ("shell-prompt", include_str!("../../../fixtures/shell-prompt/output.json")),
    ("python-repl", include_str!("../../../fixtures/python-repl/output.json")),
    ("htop", include_str!("../../../fixtures/htop/output.json")),
    ("color-showcase", include_str!("../../../fixtures/color-showcase/output.json")),
    ("ripgrep-search", include_str!("../../../fixtures/ripgrep-search/output.json")),
    ("bat-syntax", include_str!("../../../fixtures/bat-syntax/output.json")),
    ("log-viewer", include_str!("../../../fixtures/log-viewer/output.json")),
    ("editor-ui", include_str!("../../../fixtures/editor-ui/output.json")),
];

static PARSED_FIXTURES: OnceLock<Vec<TermOutput>> = OnceLock::new();

fn parsed() -> &'static Vec<TermOutput> {
    PARSED_FIXTURES.get_or_init(|| {
        FIXTURE_DATA
            .iter()
            .filter_map(|(_, json)| serde_json::from_str::<TermOutput>(json).ok())
            .collect()
    })
}

/// Get all embedded fixtures.
pub fn all_fixtures() -> &'static Vec<TermOutput> {
    parsed()
}

/// Get the first fixture (for preview cards).
pub fn default_fixture() -> Option<&'static TermOutput> {
    parsed().first()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_fixtures_parse_successfully() {
        let fixtures = all_fixtures();
        assert_eq!(
            fixtures.len(),
            FIXTURE_DATA.len(),
            "all embedded fixtures should parse"
        );
    }

    #[test]
    fn fixtures_have_nonempty_lines() {
        for f in all_fixtures() {
            assert!(!f.id.is_empty(), "fixture id should not be empty");
            assert!(!f.lines.is_empty(), "fixture {} has no lines", f.id);
        }
    }

    #[test]
    fn default_fixture_returns_first() {
        let first = default_fixture().unwrap();
        assert_eq!(first.id, all_fixtures()[0].id);
    }
}
