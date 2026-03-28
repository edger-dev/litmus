---
# litmus-3lcp
title: Research existing terminal output datasets and ANSI test suites
status: completed
type: task
priority: normal
created_at: 2026-03-24T14:01:36Z
updated_at: 2026-03-26T14:16:47Z
order: zzzV
parent: litmus-49jz
---

Research what already exists for realistic terminal output samples:

- Terminal emulator test suites (xterm.js, alacritty, kitty, wezterm test data)
- ANSI art collections and archives
- Terminal recording tools (asciinema, terminalizer) — public recording galleries
- Other theme preview tools — how do they generate sample content?
- Color scheme testing tools (base16, terminal.sexy, gogh)

Goal: find reusable content or inspiration, not reinvent. Document findings in a research note. Stage promising candidates.

## Research Findings

### Top Recommendations for Litmus

| Priority | Resource | License | Why |
|----------|----------|---------|-----|
| 1 | **tinted-theming/schemes** (GitHub) | MIT | 250+ color schemes in YAML, directly importable |
| 2 | **Gogh themes** (GitHub) | MIT | 200+ themes, clean 18-color YAML format |
| 3 | **shell-color-scripts** (GitLab dwt1) | MIT | 50+ color patterns for sample preview content |
| 4 | **terminal.sexy** (GitHub) | MIT | Template-based preview approach (captures real tmux pane content) |
| 5 | **pastel / vivid** (sharkdp, Rust) | MIT/Apache-2.0 | Color manipulation + filetype database for realistic ls output |
| 6 | **colortest** (eliminmax) | Mixed | Canonical 256-color test pattern with Rust impl |
| 7 | **asciinema asciicast v2** | Apache-2.0 | Format for capturing/replaying real terminal sessions |
| 8 | **alacritty-theme** (GitHub) | Apache-2.0 | 100+ TOML themes with screenshot previews |

### Key Findings

**Theme data sources**: tinted-theming/schemes (MIT, 250+ base16/base24 YAML) and Gogh (MIT, 200+ YAML) are the most valuable for importing color schemes. Both are permissively licensed.

**Preview content generation**: terminal.sexy uses captured tmux panes as templates. shell-color-scripts (MIT) has 50+ bash scripts producing diverse color patterns — output could be captured as sample content. vivid's filetypes.yml provides a comprehensive filetype database for realistic ls output.

**ANSI art**: 16colo.rs is the largest archive but per-artist copyright prevents bundling. Not directly usable.

**Test patterns**: colortest provides a standardized 256-color output (16 base + 216 cube + 24 grayscale) with Rust implementation. pastel has colorcheck for terminal capability detection.

**Recording format**: asciinema's asciicast v2 (NDJSON with timestamps and raw ANSI) is the standard for terminal recordings. Public recordings could supplement fixture content.
