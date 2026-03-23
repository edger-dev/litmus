---
# litmus-haaf
title: 'Fix headless screenshot capture: kitty EGL + config keys'
status: completed
type: bug
priority: normal
created_at: 2026-03-23T10:23:19Z
updated_at: 2026-03-23T10:44:16Z
---

Two blocking issues preventing screenshot capture from working:

1. Wrong kitty config keys: `initial_window_columns`/`initial_window_rows` are not valid kitty config options (logged as 'Ignoring unknown config key'). Correct keys are `initial_window_width`/`initial_window_height` (cell-count, no suffix).

2. EGL failure: kitty requires OpenGL/EGL but the wlroots headless backend (cage + WLR_BACKENDS=headless) does not expose EGL to client applications. kitty exits with '[glfw error 65542]: EGL: Failed to initialize EGL'.

Fix: Switch from cage+grim (Wayland headless) to xvfb-run+scrot (X11 virtual framebuffer). xvfb-run creates a virtual X11 display; scrot takes X11 screenshots. Mesa software rendering (LIBGL_ALWAYS_SOFTWARE=1) works on X11 Xvfb with no GPU.

## Tasks
- [x] Fix kitty config: initial_window_columns → initial_window_width, initial_window_rows → initial_window_height
- [x] Switched approach: cage+grim with foot terminal (no OpenGL required) instead of xvfb+kitty
- [x] Update flake.nix: added foot, grim, cage; foot config format fixed for v1.26.1
- [x] Update screenshots.yml: uses nix devshell (cage+grim+foot), removed xvfb/mesa deps

## Summary of Changes

- **Root cause**: Two separate issues blocked headless capture:
  1. Kitty config used `initial_window_columns`/`initial_window_rows` (invalid) → fixed to `initial_window_width`/`initial_window_height`
  2. Kitty requires OpenGL/EGL which wlroots headless backend doesn't expose → switched to foot terminal (renders via Wayland SHM, no OpenGL)
  
- **Architecture**: cage (headless Wayland) + foot (SHM terminal) + grim (Wayland screenshot)
  - foot is pure Wayland, renders via shared-memory buffers — works perfectly with WLR_RENDERER=pixman
  - FootProvider generates foot 1.26.1 config with [colors-dark] section
  - cursor color uses double-value format: `cursor = <bg> <text>`

- **Additional fixes**:
  - Fixture command paths now canonicalized to absolute (was failing after `cd` to work dir)
  - FIXTURE_WORK_DIR exported to terminal environment
  - git-diff and git-log fixtures now use `git --no-pager` to prevent pager from blocking

- **Verified**: End-to-end capture of tokyo-night + git-diff and ls-color produces valid WebP screenshots
