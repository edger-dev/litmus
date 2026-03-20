---
# litmus-e29y
title: Compare accumulator with floating bar
status: completed
type: feature
priority: normal
created_at: 2026-03-20T18:29:16Z
updated_at: 2026-03-20T18:39:33Z
parent: litmus-74j8
---

Global 'Add to compare' button stores theme slug in context signal. Floating bottom bar shows selected themes, navigates to compare when 2+ selected.

## Summary of Changes

Added global compare selection using Dioxus context signal (CompareSelection). Added CompareToggle button on theme cards and detail page. Added floating CompareBar at the bottom showing selected themes as chips with remove buttons, a 'Go to Compare' link (when 2+ selected), and a Clear button. CSS for .compare-toggle, .compare-bar, .compare-chip, and .compare-bar-btn. Max 4 themes can be selected.
