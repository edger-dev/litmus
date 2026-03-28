---
# litmus-c55s
title: Build color swatch and color showcase fixtures
status: completed
type: task
priority: normal
created_at: 2026-03-24T14:01:30Z
updated_at: 2026-03-26T14:16:47Z
order: zy
parent: litmus-49jz
---

Create two special-purpose fixtures:

**color-swatch/** — raw reference palette:
- Small Rust or shell program
- Prints all 16 ANSI colors (0-15) as labeled foreground text + background blocks
- 256-color palette grid (6x6x6 cube + 24 grayscale)
- A few truecolor gradients (red→green, dark→light, hue wheel)
- Must fit 80x24

**color-showcase/** — themed palette in context:
- Looks like a real scenario (e.g. status dashboard, CI pipeline summary)
- Naturally exercises all 16 ANSI colors including brights
- Uses background colors for status bars/highlights
- Deterministic output

## Plan

### color-swatch fixture
A shell script that prints a reference palette:
- 16 ANSI colors as labeled foreground text and background blocks
- 256-color palette grid (6×6×6 cube sampling + grayscale)
- A few truecolor gradient samples
- setup.sh is empty (no state needed), command.sh does all output

### color-showcase fixture
A simulated CI/deploy dashboard that naturally uses all 16 ANSI colors:
- Build status lines (green pass, red fail, yellow warning)
- Deploy pipeline stages
- Service health checks
- Deterministic, self-contained printf/echo output

### Todo
- [x] Create color-swatch fixture (setup.sh + command.sh)
- [x] Create color-showcase fixture (setup.sh + command.sh)
- [x] Test both fixtures locally
- [x] Update README.md fixture inventory
- [x] Review

## Summary of Changes

Created two new fixtures:

- **color-swatch**: Reference palette with 16 ANSI colors (fg labels + bg blocks), full 256-color cube, grayscale ramp, and truecolor gradient. 19 lines.
- **color-showcase**: Simulated CI deploy dashboard exercising all 16 standard and bright ANSI colors through build, deploy, health check, and error sections. 20 lines.

Both are pure printf output (no setup needed), deterministic, fit 80x24, and parse cleanly through the ANSI parser. Updated fixtures/README.md inventory.
