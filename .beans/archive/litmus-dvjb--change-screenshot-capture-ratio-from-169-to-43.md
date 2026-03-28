---
# litmus-dvjb
title: Change screenshot capture ratio from 16:9 to 4:3
status: completed
type: task
priority: normal
created_at: 2026-03-25T15:38:34Z
updated_at: 2026-03-26T14:16:47Z
order: F
---

Current screenshots are 1280x720 (16:9), which is too wide for terminal content — leaves dead space on the right. Switch to 4:3 (e.g. 960x720 or 1024x768) for a more natural terminal aspect ratio.

## Tasks
- [x] Decide exact capture dimensions: 80 cols × 32 rows → ~1280×960 (4:3 at 2x)
- [x] Update capture config/code with new terminal window size
- [x] Fix display resolution via wlr-randr (WLR_SCREEN_SIZE removed in wlroots 0.19)
- [ ] Re-capture all screenshots for both providers (kitty, wezterm)
- [ ] Rebuild manifest
- [ ] Verify screenshots look good at new ratio

## Implementation

The root cause was that `grim` captures the entire Wayland display managed by `cage`, not just the terminal window. Changing `TermGeometry` rows only affected terminal cell config, not the compositor resolution.

Fix: Set `WLR_SCREEN_SIZE` environment variable on the `cage` command in `capture.rs` to `{pixel_width}x{pixel_height}` from the geometry. Added `pixel_width` and `pixel_height` fields to `TermGeometry` (default 1280x960 = 4:3).

## Notes

- wlroots 0.19+ ignores `WLR_SCREEN_SIZE` env var — used `wlr-randr --custom-mode` inside the cage wrapper script instead
- Added `pixel_width`/`pixel_height` to `TermGeometry` (default 1280x960 = 4:3)
- Added `wlr-randr` to nix devshell dependencies

Re-capture requires Wayland + GPU environment:
```bash
mise run capture-kitty
mise run capture-wezterm
mise run capture-manifest
```
Then verify the new screenshots have ~4:3 ratio and look good.
