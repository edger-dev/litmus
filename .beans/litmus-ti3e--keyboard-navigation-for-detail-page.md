---
# litmus-ti3e
title: Keyboard navigation for detail page
status: completed
type: task
priority: normal
created_at: 2026-03-20T18:29:16Z
updated_at: 2026-03-20T18:40:30Z
parent: litmus-74j8
---

ArrowLeft/Right to switch scene tabs, 'c' to toggle compare selection. onkeydown handler on outer div.

## Summary of Changes

Added keyboard navigation to the detail page: ArrowLeft/ArrowRight switches scene tabs, 'c' toggles compare selection. The outer div is made focusable with tabindex and autofocus. A subtle keyboard hint is shown next to the scene tabs.
