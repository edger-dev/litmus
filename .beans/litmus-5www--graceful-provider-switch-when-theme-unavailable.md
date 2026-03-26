---
# litmus-5www
title: Graceful provider switch when theme unavailable
status: completed
type: feature
priority: normal
created_at: 2026-03-26T14:51:24Z
updated_at: 2026-03-26T16:29:53Z
blocked_by:
    - litmus-3svg
---

When viewing a theme detail page and switching to a provider that doesn't have the theme, currently loads a blank "Theme not found" page. Instead, stay on the current page and show an inline alert.

## Requirements

- [x] When provider switch is requested but theme is unavailable for target provider, stay on current page
- [x] Show an alert/banner on the current page: "{theme} is not available for {provider}"
- [x] Add provider availability metadata to the detail page (which providers have this theme) so the UI can check before navigating
- [x] Provider selector in sidebar should indicate which providers have the current theme (e.g. dim/disable unavailable ones, or show a badge)
- [x] Dismiss alert on user action or after timeout

## Design Notes

Key insight: the user is already on the page viewing the theme. Don't navigate away and fail — prevent the bad navigation and inform inline.


## Summary of Changes

Implemented graceful provider switching:
- Provider buttons dim when theme is unavailable for that provider
- Clicking an unavailable provider shows a dismissible alert banner instead of navigating
- Alert auto-dismisses after 3s or on manual dismiss (x button)
- Added `theme_available_for_provider()` helper, `AlertMessage` state, and `AlertBanner` component

Key files: `sidebar.rs` (availability check + button logic), `shell.rs` (AlertBanner), `state.rs` (AlertMessage), `themes.rs` (helper), `style.css` (styles)
