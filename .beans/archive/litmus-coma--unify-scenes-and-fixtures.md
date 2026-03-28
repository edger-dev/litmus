---
# litmus-coma
title: Unify scenes and fixtures
status: completed
type: epic
priority: normal
created_at: 2026-03-23T15:17:32Z
updated_at: 2026-03-26T14:16:47Z
order: s
---

## Goal

Replace hand-written scenes with parsed real ANSI output from fixtures. One source of truth (fixture scripts) drives both the simulated view (spans) and screenshots. Simulated view becomes the primary rendering, screenshots become validation.

**Key decisions:**
- Fixtures → Scenes direction (real output is ground truth, not hand-written)
- Span-based storage, not cell grid (compact, ligature-friendly, fast HTML rendering)
- TermColor supports full spectrum: Default, Ansi(0-15), Indexed(16-255), Rgb(r,g,b)
- Output files are per-provider (aligns with litmus-knrz provider-scoped rendering)
- Capture pipeline tees ANSI stream and parses it alongside screenshot capture

## Data Model

```rust
enum TermColor {
    Default,              // theme fg or bg
    Ansi(u8),             // 0-15, resolved from theme
    Indexed(u8),          // 16-255, fixed RGB
    Rgb(u8, u8, u8),      // 24-bit truecolor, literal
}

struct TermSpan {
    text: String,
    fg: TermColor,
    bg: TermColor,
    bold: bool,
    italic: bool,
    dim: bool,
    underline: bool,
}

struct TermLine {
    spans: Vec<TermSpan>,
}

struct TermOutput {
    id: String,           // fixture id
    name: String,         // display name
    cols: u16,
    rows: u16,
    lines: Vec<TermLine>,
}
```

## Parsing Pipeline

```
fixture command.sh → raw ANSI stream → VTE parser → intermediate cell grid → collapse to spans → TermOutput → output.{provider}.json
```

## File Layout

```
fixtures/git-diff/
    setup.sh              # existing, unchanged
    command.sh            # existing, unchanged
    output.kitty.json     # generated — parsed TermOutput from kitty capture
    output.wezterm.json   # generated — parsed TermOutput from wezterm capture
```

## Web App Rendering

- TermOutput replaces Scene as the renderable unit
- TermColor resolution: Default → theme fg/bg, Ansi(n) → ProviderColors palette, Indexed(n) → fixed 256-color table, Rgb → literal CSS
- Provider selector switches both simulated view (output.{provider}.json) and screenshot
- Contrast validation updated for TermColor (theme-dependent pairs validated, fixed colors checked against theme bg)

## Subtasks (in dependency order)

**Unblocked:**
1. `litmus-q9lp` — Add TermColor, TermSpan, TermLine, TermOutput types to litmus-model

**Blocked by 1:**
2. `litmus-28sq` — Build ANSI-to-spans parser using VTE

**Blocked by 2:**
3. `litmus-9eg8` — Integrate ANSI capture into fixture pipeline

**Blocked by 1 + 3 (can run in parallel):**
4. `litmus-lm76` — Update litmus-web to render TermOutput instead of Scene
5. `litmus-0uoe` — Update litmus-cli to render TermOutput

**Blocked by 1 + 4:**
6. `litmus-bcel` — Update contrast validation for TermColor

**Blocked by 4 + 5 + 6:**
7. `litmus-kbzo` — Remove old Scene, ThemeColor, StyledSpan types and hand-written scenes

## Summary of Changes

All 7 subtasks completed. The codebase now uses TermOutput (parsed from real ANSI output) as the sole rendering model. Scene/ThemeColor/StyledSpan types have been fully removed. The TermOutput data model supports Default, Ansi(0-15), Indexed(16-255), and Rgb colors with bold/italic/dim/underline modifiers. Both litmus-cli and litmus-web render TermOutput fixtures, and contrast validation uses the new TermColor-based analysis.
