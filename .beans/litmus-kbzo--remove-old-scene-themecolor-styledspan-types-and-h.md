---
# litmus-kbzo
title: Remove old Scene, ThemeColor, StyledSpan types and hand-written scenes
status: todo
type: task
priority: normal
created_at: 2026-03-24T13:47:38Z
updated_at: 2026-03-24T13:47:44Z
parent: litmus-coma
blocked_by:
    - litmus-lm76
    - litmus-0uoe
    - litmus-bcel
---

Final cleanup once all consumers migrated to TermOutput:

- Delete scene.rs (ThemeColor, StyledSpan, SceneLine, Scene)
- Delete scenes.rs (hand-written scene definitions)
- Remove scene_id_to_fixture_id() mapping (no longer needed)
- Clean up any remaining references to old types
- Verify all tests pass

Depends on: web and CLI migrations, contrast validation update
