---
# litmus-q9lp
title: Add TermColor, TermSpan, TermLine, TermOutput types to litmus-model
status: completed
type: task
priority: normal
created_at: 2026-03-24T13:47:13Z
updated_at: 2026-03-26T14:16:47Z
order: zV
parent: litmus-coma
---

Add the new terminal output types to litmus-model:

- TermColor enum: Default, Ansi(u8), Indexed(u8), Rgb(u8, u8, u8)
- TermSpan: text + fg/bg TermColor + bold/italic/dim/underline
- TermLine: Vec<TermSpan>
- TermOutput: id, name, cols, rows, Vec<TermLine>
- Serde JSON serialization/deserialization for all types
- TermColor resolution method: resolve(provider_colors) → CSS-ready rgb values
- Indexed(16-255) → fixed RGB lookup table (standard 256-color palette)

Keep existing Scene/ThemeColor types — removed in a later task.

## Plan

### New file: `crates/litmus-model/src/term_output.rs`

1. `TermColor` enum: Default, Ansi(u8), Indexed(u8), Rgb(u8, u8, u8)
2. `TermColor::resolve()` method: takes ProviderColors, returns Color
   - Default → uses context (caller decides fg vs bg)
   - Ansi(0-15) → ProviderColors.ansi lookup
   - Indexed(16-255) → fixed 256-color palette lookup table
   - Rgb → direct Color
3. `TermSpan`: text + fg/bg TermColor + bold/italic/dim/underline
4. `TermLine`: Vec<TermSpan>
5. `TermOutput`: id, name, cols, rows, Vec<TermLine>
6. Serde JSON serialization/deserialization for all types

### 256-color palette
Standard xterm-256 color palette: colors 16-231 are a 6x6x6 color cube, 232-255 are grayscale.

### Commits
1. Tests + implementation
2. Review fixes

## Summary of Changes

Added `term_output` module to litmus-model with:

- **`TermColor`** enum: Default, Ansi(0-15), Indexed(16-255), Rgb(r,g,b) with tagged serde
- **`TermColor::resolve()`**: maps to concrete Color via ProviderColors + default fallback
- **Standard xterm-256 color palette**: 6x6x6 cube (16-231) + grayscale ramp (232-255)
- **`TermSpan`**: text + fg/bg TermColor + bold/italic/dim/underline with skip_serializing_if
- **`TermLine`** and **`TermOutput`**: structured parsed terminal output
- 11 tests covering resolution, palette corners, and JSON serde round-trips

Existing Scene/ThemeColor types kept intact.
