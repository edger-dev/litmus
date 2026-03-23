---
# litmus-jr2u
title: Expand themes collection to ~60 themes
status: completed
type: task
priority: normal
created_at: 2026-03-22T17:44:37Z
updated_at: 2026-03-23T10:26:52Z
order: zzzs
---

Add ~30 new well-known themes from verified sources: GitHub, One Light, Night Owl, Iceberg, Snazzy, Zenburn, Flexoki, Oxocarbon, Poimandres, Cyberdream, Vesper, Modus, Nightfox family, Sonokai, Andromeda, Melange, Tender


## Summary of Changes

Added 31 new themes from well-known, verified sources (official repos, kitty-themes, alacritty-theme, iTerm2-Color-Schemes):

**New theme families (with dark/light variants):**
- GitHub (Dark, Light, Dark Dimmed) — from projekt0n/github-theme-contrib
- Cyberdream (Dark, Light) — from scottmckendry/cyberdream.nvim
- Flexoki (Dark, Light) — from kepano/flexoki
- Iceberg (Dark, Light) — from cocopon/iceberg.vim
- Melange (Dark, Light) — from savq/melange-nvim
- Modus (Vivendi, Operandi) — from GNU Emacs modus-themes
- Night Owl + Light Owl — from sdras/night-owl-vscode-theme
- Oxocarbon (Dark, Light) — from nyoom-engineering/oxocarbon
- Sonokai (Default, Shusia) — from sainnhe/sonokai

**New Nightfox family members:**
- Dawnfox, Dayfox, Duskfox, Nordfox, Terafox — from EdenEast/nightfox.nvim
- Moved existing nightfox.toml into nightfox/ directory

**New standalone themes:**
- Andromeda — from EliverLara/Andromeda
- One Light — Atom One Light terminal port
- Poimandres — from drcmda/poimandres-theme
- Snazzy — from sindresorhus/hyper-snazzy
- Tender — from jacoborus/tender.vim
- Vesper — from raunofreiberg/vesper
- Zenburn — classic vim theme

**Code changes:**
- Updated themes.rs with 60 include_str! entries (was 29)
- Updated family.rs with new prefix families + suffix-based grouping for Nightfox and Owl families
- All tests pass

Total: 29 → 60 themes
