---
# litmus-k6ah
title: Docs update and spec brainstorming
status: completed
type: task
priority: normal
created_at: 2026-03-27T15:28:16Z
updated_at: 2026-03-27T15:58:16Z
---

Restructure and rewrite the mdbook docs site to serve end users and contributors.
Current docs are substantially out of date since v0.3.0 and structured around developer internals.

## Approved Structure

```
Introduction              — elevator pitch, live link, 60 themes / 13 fixtures / 2 providers
The Model                 — provider/consumer/silo conceptual framework (centerpiece)
  ├─ Providers            — terminal emulators as color palette sources
  ├─ Consumers            — apps that inherit ANSI colors, fixtures as representations
  └─ Silos (Roadmap)      — apps with independent themes (neovim etc), clearly future
Using Litmus              — end-user guide
  ├─ Browsing Themes      — filters, search, dark/light, provider switching
  ├─ Theme Detail         — fixtures, contrast chips, screenshot vs simulated
  ├─ Comparing Themes     — side-by-side, issue navigation, favorites
  └─ Exporting Config     — kitty.conf, TOML, Nix
Under the Hood
  ├─ Capture Pipeline     — litmus-capture, headless terminals, ANSI → WebP → R2
  ├─ Rendering            — TermOutput model, TermColor resolution, dual-mode display
  ├─ Accessibility        — WCAG contrast, APCA scoring, CVD simulation
  └─ Architecture         — 4 crates, workspace layout, data flow, deployment
Contributing
  ├─ Dev Setup            — nix, mise, bacon workflow
  ├─ Adding Themes        — updated from current
  ├─ Adding Fixtures      — updated from current
  └─ Adding Providers     — new
Roadmap                   — replaces milestones + next-stage-plan, shipped vs planned
```

## Pages Removed
- `milestones.md` → compressed into Roadmap
- `next-stage-plan.md` → absorbed into Roadmap (shipped vs planned)
- `agentic-workflow.md` → bacon/beans details move to Dev Setup; rest cut

## The Model (Centerpiece)

The page that makes litmus click conceptually. ~100-150 lines, explanatory not academic.

1. **Opening hook** — "Why switching terminal themes is broken" — framed through color ownership. The pain isn't just manual config — it's that different apps get colors from different places, and you can't see the full picture until everything is configured.

2. **The three roles:**
   - **Providers** — terminal emulators defining 16 ANSI palette + fg/bg/cursor/selection. Source of truth. Changing a provider's theme changes everything inside it. Litmus supports kitty and wezterm.
   - **Consumers** — apps inheriting the provider's palette via ANSI codes. Don't define colors, reference by index. `git diff` says "make additions green" (ANSI 2), and what "green" looks like depends on the provider. Why the same `git diff` looks different under Tokyo Night vs Gruvbox.
   - **Silos** — apps with their own color palette, independent of provider. Neovim colorschemes, lazygit built-in themes. *(Roadmap — planned for next major version, not yet in litmus.)*

3. **Ecosystems** — provider + consumers as a natural group. A theme preview showing only swatches is useless — you need to see the ecosystem. This is litmus's core insight.

4. **Dual-mode apps** — brief note: some apps (lazygit, jjui) can be consumer or silo. *(Roadmap.)*

5. **What litmus does with this** — captures real terminal output from real providers. Not simulation of what git diff *might* look like — actual pixels your terminal would render.

## Under the Hood

### Capture Pipeline
- **Problem solved:** simulated rendering can't capture font rendering, ligatures, actual app behavior. Litmus captures real screenshots from headless Wayland terminals.
- **Flow:** litmus-capture writes temp provider config (80×32, 12pt FiraCode, 1280×960 at 2x) → runs fixture setup.sh + command.sh → launches terminal headlessly → screenshots with grim → PNG→WebP + SHA-256 checksum → repeat for every (provider × theme × fixture) combination.
- **ANSI parsing:** also parses raw ANSI escapes into structured TermOutput for the simulated rendering path. Two display modes: real screenshots and simulated.
- **Manifest & CDN:** Cloudflare R2, manifest.json indexes all screenshots. Web app fetches manifest once, lazy-loads images. Cache-busted URLs (1yr TTL images, 1min manifest).

### Rendering
- **TermOutput model:** replaces old Scene system. TermColor (Default/Ansi/Indexed/Rgb) → TermSpan (text + fg + bg) → lines. Models real terminal output including 256-color and truecolor.
- **Color resolution:** TermColor::resolve() maps semantic refs to RGB via ProviderColors. Ansi(2) → provider's "green". Rgb passes through.
- **Dual display:** detail page shows real screenshots (R2) alongside simulated rendering (ANSI → TermOutput → CSS). Screenshots are ground truth; simulated enables CVD sim and contrast analysis.

### Accessibility
- **WCAG contrast:** contrast.rs computes ratios via WCAG 2.1 relative luminance. AA (4.5:1) and AAA (7:1).
- **APCA scoring:** perceptually accurate readability beyond WCAG ratio.
- **Per-fixture analysis:** every span checked. Issues surface as interactive chips — click to cycle.
- **CVD simulation:** Machado 2009 matrices transform entire palettes. Applied at ProviderColors level.

### Architecture
- **Four crates:** litmus-model (shared types, any target), litmus-web (Dioxus WASM), litmus-cli (ratatui TUI, secondary), litmus-capture (headless capture, native Wayland).
- **Data flow:** theme TOML → ThemeDefinition + ProviderColors → compile-time embed → fixtures from output.json → TermColor resolves against ProviderColors → CSS. Screenshots: R2 manifest → lazy images.
- **Deployment:** Cloudflare Pages via GitHub Actions. dx build → critical CSS injection → Wrangler deploy. Screenshots on R2 subdomain.
- **Provider-scoped routing:** all routes under /:provider/ — same theme shows different data per provider.

## Using Litmus
- **Browsing:** 60 themes grouped by family, search, dark/light filter, readability threshold, provider toggle (kitty/wezterm), greyed unavailable themes.
- **Detail:** 13 fixtures, dual display (screenshot + simulated), contrast chips, fixture minimap, sticky toolbar.
- **Comparing:** 2-theme side-by-side, inline pickers, per-fixture issue dots, "vs." memory, favorites (20 cap, star toggle).
- **Export:** kitty.conf, TOML, Nix. Copy or download.

## Contributing
- **Dev Setup:** Rust stable + wasm32, Nix optional, mise required. `mise run dev`, bacon workflow, beans for tracking.
- **Adding Themes:** updated for ThemeDefinition + per-provider color files.
- **Adding Fixtures:** updated for capture-based system (setup.sh + command.sh + output.json).
- **Adding Providers:** new page — config generator in litmus-capture/src/providers/, color extraction, manifest pipeline.

## Roadmap
- **Shipped:** v0.1–v0.5 compressed summary, link to CHANGELOG.md.
- **Next major:** silo support (neovim colorschemes, dual-mode), ecosystem view, extended export (alacritty, foot, ghostty, Windows Terminal, iTerm2), consumer config preview.
- **Not in scope:** live terminal capture in browser, theme editor, plugin system, user accounts.

## Tasks
- [x] Write SUMMARY.md (new structure)
- [x] Write Introduction (updated stats, live link)
- [x] Write The Model (centerpiece, expanded from concepts.md)
- [x] Write Using Litmus (4 subsections)
- [x] Write Under the Hood: Capture Pipeline
- [x] Write Under the Hood: Rendering
- [x] Write Under the Hood: Accessibility
- [x] Write Under the Hood: Architecture (4 crates, data flow, deployment)
- [x] Write Contributing: Dev Setup (absorb agentic-workflow bacon/beans bits)
- [x] Update Contributing: Adding Themes
- [x] Update Contributing: Adding Fixtures
- [x] Write Contributing: Adding Providers (new)
- [x] Write Roadmap (replace milestones + next-stage-plan)
- [x] Remove stale pages (milestones.md, next-stage-plan.md, agentic-workflow.md)
- [x] Rebuild docs/dist


## Summary of Changes

Rewrote the entire mdbook documentation site with a new structure targeting end users and contributors:

- **The Model** (centerpiece) — expanded provider/consumer/silo conceptual framework
- **Using Litmus** — 4-page end-user guide (browsing, detail, comparing, exporting)
- **Under the Hood** — 4-page technical deep dive (capture pipeline, rendering, accessibility, architecture)
- **Contributing** — 4-page contributor guide (dev setup, adding themes/fixtures/providers)
- **Roadmap** — replaces stale milestones and next-stage-plan pages

Removed 6 stale pages (milestones, next-stage-plan, agentic-workflow, concepts, architecture, development). Fixed all technical inaccuracies found during code review (TermColor::Indexed type, color count, route params, font name, theme count).
