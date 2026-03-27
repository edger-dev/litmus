---
# litmus-4ai1
title: Cap compare at 3 themes
status: todo
type: task
created_at: 2026-03-27T04:53:18Z
updated_at: 2026-03-27T04:53:18Z
parent: litmus-ysy5
---

Enforce a maximum of 3 themes in side-by-side compare.

- [ ] Change MAX_COMPARE from 4 to 3 in state.rs
- [ ] Truncate slug list to 3 in URL parsing
- [ ] Update sidebar compare button to respect the cap
- [ ] Show a brief message or toast if user tries to add a 4th theme
- [ ] Ensure CSS grid adapts well to 2 and 3 columns with contrast markers
