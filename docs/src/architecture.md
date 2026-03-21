# Architecture

litmus is a Rust workspace with three crates that share a common data model.

## Workspace layout

```
crates/
  litmus-model/   — shared data model, theme parsing, scenes, validation
  litmus-cli/     — TUI binary (ratatui + crossterm) — the M0 prototype
  litmus-web/     — web frontend (Dioxus, targets wasm32) — the primary interface
```

Both `litmus-cli` and `litmus-web` depend on `litmus-model`. The model crate has no UI dependencies and compiles for any target.

## Data model (`litmus-model`)

### Core types

The model centers on three types in `lib.rs`:

- **`Color`** — an RGB triplet (`r: u8, g: u8, b: u8`) with `to_hex()` / `from_hex()` conversion.
- **`AnsiColors`** — the 16 named ANSI terminal colors as individual fields (`black`, `red`, ..., `bright_white`). Provides `from_array()` / `as_array()` for indexed access.
- **`Theme`** — the complete theme definition: `name`, `background`, `foreground`, `cursor`, `selection_background`, `selection_foreground`, and an `AnsiColors` struct.

### Theme format

Themes are TOML files with a flat structure:

```toml
name = "Dracula"

[colors]
background = "#282a36"
foreground = "#f8f8f2"
cursor = "#f8f8f2"
selection_background = "#44475a"
selection_foreground = "#f8f8f2"

[colors.ansi]
black = "#21222c"
red = "#ff5555"
# ... all 16 ANSI colors
```

The `toml_format` module handles parsing. Additional parsers exist for kitty and base16 formats. Exporters can write themes as kitty.conf, TOML, or Nix attribute sets.

### Theme embedding

Themes are embedded at compile time. In `crates/litmus-web/src/themes.rs`, a static array uses `include_str!` to inline each theme's TOML:

```rust
static THEME_DATA: &[&str] = &[
    include_str!("../../../themes/ayu/dark.toml"),
    include_str!("../../../themes/ayu/light.toml"),
    // ...
];
```

`load_embedded_themes()` parses these strings at startup and returns a sorted `Vec<Theme>`.

### Theme families

Themes are grouped into families by name prefix. The `family.rs` module defines known family prefixes:

```rust
static FAMILIES: &[&str] = &[
    "Ayu", "Catppuccin", "Everforest", "Gruvbox", "Kanagawa",
    "Rose Pine", "Rosé Pine", "Solarized", "Tokyo Night",
];
```

`group_by_family()` partitions themes — those matching a prefix are grouped together, the rest become single-theme families. This drives the grouped layout on the home page.

## Scene system

The scene system is the key abstraction that makes litmus work. Scenes simulate real terminal output using semantic color references instead of hardcoded values.

### `ThemeColor` enum

```rust
pub enum ThemeColor {
    Foreground,
    Background,
    Cursor,
    SelectionBackground,
    SelectionForeground,
    Ansi(u8),  // 0–15
}
```

`ThemeColor::resolve(&self, theme: &Theme) -> &Color` maps a semantic reference to the actual color in a given theme. This indirection means a single scene definition renders correctly across all themes.

### Composition

- **`StyledSpan`** — text + optional fg/bg `ThemeColor` + style flags (bold, italic, dim, underline). Built with `StyledSpan::plain()`, `StyledSpan::colored()`, and chainable `.bold()`, `.dim()`, `.italic()`, `.on(bg)`.
- **`SceneLine`** — a vector of `StyledSpan`s representing one terminal row.
- **`Scene`** — id, name, description, and a vector of `SceneLine`s.

### Built-in scenes

The 8 scenes in `scenes.rs` simulate common terminal contexts:

| Scene | Simulates |
|-------|-----------|
| Shell Prompt | Bash/zsh prompt with git branch |
| Git Diff | Unified diff output with adds/removes |
| LS Colors | Colorized directory listing |
| Cargo Build | Rust compiler errors and warnings |
| Log Viewer | Structured log output with levels |
| Neovim | Editor UI with line numbers and syntax |
| Python REPL | Interactive Python session |
| htop | System monitor with resource bars |

### Rendering flow

1. A scene defines styled spans with `ThemeColor` references
2. The renderer (web or TUI) iterates spans
3. Each `ThemeColor` is resolved against the current `Theme` to get an RGB `Color`
4. The resolved color is applied as a CSS inline style (web) or terminal escape (TUI)

## Web app (`litmus-web`)

The web frontend is a Dioxus WASM application.

### Router

Four routes handle the main views:

| Route | Component | Purpose |
|-------|-----------|---------|
| `/` | `ThemeList` | Home — filterable grid of theme cards |
| `/theme/:slug` | `ThemeDetail` | Single theme with scene tabs, palette, export |
| `/scene/:scene_id` | `SceneAcrossThemes` | One scene rendered across all themes |
| `/compare/:slugs` | `CompareThemes` | Side-by-side comparison (2–4 themes) |

### Component hierarchy

```
App                        — context providers (CompareSelection)
└── Shell                  — navigation bar + content area
    ├── ThemeList           — search, filters, family-grouped cards
    │   └── ThemeCard       — swatch strip + mini scene preview
    ├── ThemeDetail         — scene tabs, palette, export buttons
    │   ├── AllScenesView   — renders all scenes for a theme
    │   └── ExportButtons   — kitty.conf / TOML / Nix export
    ├── SceneAcrossThemes   — grid of one scene across themes
    └── CompareThemes       — 2–4 themes side by side
```

### Scene rendering

`scene_renderer.rs` resolves `ThemeColor` references to CSS inline styles. The `SceneView` component renders a scene as a `<pre>` block, applying `color` and `background-color` from the resolved theme colors, plus `font-weight`, `font-style`, and `opacity` for text styles.

### Filters and features

- **Search**: filters themes by name or family
- **Variant filter**: All / Dark / Light (based on background luminance)
- **Contrast filter**: only themes passing WCAG AA readability checks
- **CVD simulation**: view themes as seen with protanopia, deuteranopia, or tritanopia
- **Compare accumulator**: press 'c' or click to collect up to 4 themes for comparison
- **Keyboard navigation**: arrow keys to cycle scenes on the detail page

### Styling

`style.css` uses CSS custom properties for layout tokens and provides responsive breakpoints for mobile. The actual theme colors are applied as inline styles by the scene renderer, not through CSS classes.

## Accessibility and CVD

### WCAG contrast checking

`contrast.rs` implements WCAG 2.1 contrast ratio calculation:

- `relative_luminance()` converts sRGB to linear luminance
- `contrast_ratio()` computes the ratio between two colors
- `validate_theme_readability()` checks a theme's foreground/background against WCAG AA (4.5:1) and AAA (7.0:1) thresholds
- `validate_scene_contrast()` checks every span in a scene against the theme

The contrast filter on the home page uses `validate_theme_readability()` to surface only themes with adequate contrast.

### CVD simulation

`cvd.rs` simulates color vision deficiency using Machado et al. 2009 transformation matrices:

- **Protanopia** — reduced red sensitivity
- **Deuteranopia** — reduced green sensitivity
- **Tritanopia** — reduced blue sensitivity

The simulation converts sRGB to linear RGB, applies a 3x3 transformation matrix, and converts back. `simulate_theme()` applies the transform to every color in a theme, producing a new `Theme` that can be rendered normally through the existing scene pipeline.

### Semantic HTML and ARIA

The web app uses semantic HTML elements and ARIA attributes for screen reader compatibility. Interactive elements have visible focus indicators via `:focus-visible` styles. The layout is responsive down to mobile viewports.
