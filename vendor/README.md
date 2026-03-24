# Vendored Provider Theme Data

This directory contains color scheme data from terminal emulator providers.
These files are the source of truth for `litmus extract-colors`.

## kitty-themes

**Source:** https://github.com/kovidgoyal/kitty-themes
**Method:** `git subtree`
**Format:** `.conf` (kitty config format)
**Count:** ~385 theme files

### Updating

```bash
git subtree pull --prefix vendor/kitty-themes https://github.com/kovidgoyal/kitty-themes.git master --squash
```

## wezterm-colorschemes

**Source:** https://github.com/wez/wezterm (`config/src/scheme_data.rs`)
**Method:** One-time extraction from embedded Rust data
**Format:** `.toml` (wezterm native TOML)
**Count:** ~1001 color schemes

The wezterm repo embeds all color schemes as TOML strings inside a single Rust
source file. We extract them into individual `.toml` files for easier consumption.

### Updating

1. Download the latest `scheme_data.rs`:
   ```bash
   curl -sL "https://raw.githubusercontent.com/wez/wezterm/main/config/src/scheme_data.rs" -o /tmp/scheme_data.rs
   ```

2. Run the extraction script:
   ```bash
   python3 scripts/extract-wezterm-schemes.py /tmp/scheme_data.rs vendor/wezterm-colorschemes/schemes
   ```

3. Commit the updated files.
