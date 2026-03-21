# Adding Themes

This guide walks through adding a new terminal color theme to litmus.

## 1. Create the TOML file

Themes live in the `themes/` directory. Standalone themes go directly in `themes/`, while family members go in a subdirectory:

```
themes/dracula.toml          # standalone
themes/catppuccin/mocha.toml  # family member
```

Every theme TOML has the same structure — a `name`, a `[colors]` table with special colors, and a `[colors.ansi]` table with the 16 ANSI colors:

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
green = "#50fa7b"
yellow = "#f1fa8c"
blue = "#bd93f9"
magenta = "#ff79c6"
cyan = "#8be9fd"
white = "#f8f8f2"
bright_black = "#6272a4"
bright_red = "#ff6e6e"
bright_green = "#69ff94"
bright_yellow = "#ffffa5"
bright_blue = "#d6acff"
bright_magenta = "#ff92df"
bright_cyan = "#a4ffff"
bright_white = "#ffffff"
```

All color values are hex strings (`#RRGGBB`). Every field is required.

## 2. Register in `themes.rs`

Open `crates/litmus-web/src/themes.rs` and add an `include_str!` entry to the `THEME_DATA` array:

```rust
static THEME_DATA: &[&str] = &[
    // ... existing themes ...
    include_str!("../../../themes/dracula.toml"),
];
```

The path is relative to the `themes.rs` file. For family themes in subdirectories:

```rust
include_str!("../../../themes/catppuccin/mocha.toml"),
```

This embeds the theme at compile time — no runtime file I/O needed.

## 3. If adding a new family

If your theme belongs to a family that doesn't exist yet (e.g. you're adding the first "Kanagawa" variant), register the family prefix in `crates/litmus-web/src/family.rs`:

```rust
static FAMILIES: &[&str] = &[
    "Ayu",
    "Catppuccin",
    // ... add your family name here ...
    "Kanagawa",
];
```

The family grouping uses prefix matching — a theme named "Kanagawa Wave" will match the "Kanagawa" family prefix.

## 4. Verify

1. Run `cargo check` (or read `.bacon-claude-diagnostics` if bacon is running)
2. Start the web app with `mise run dev`
3. Confirm your theme appears in the theme list
4. Check that all scenes render correctly on the theme detail page
5. If the theme is part of a family, verify it groups correctly
