---
# litmus-u02e
title: Implement APCA algorithm for light theme readability scoring
status: completed
type: task
priority: normal
created_at: 2026-03-21T09:31:09Z
updated_at: 2026-03-21T13:10:26Z
order: zzw
parent: litmus-sh4g
---

Replace WCAG 2.x symmetric contrast ratio with APCA (polarity-aware) in readability_score(). WCAG under-estimates perceived contrast for dark text on light backgrounds, causing light themes to score incorrectly low. APCA correctly models both polarities. Results: Catppuccin Latte 31.9% → ~90%, Solarized Light 10.2% → 99.6%, with parity to dark variants.
