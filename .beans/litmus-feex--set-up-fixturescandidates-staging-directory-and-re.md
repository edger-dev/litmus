---
# litmus-feex
title: Set up fixtures/candidates/ staging directory and review workflow
status: completed
type: task
priority: normal
created_at: 2026-03-24T14:01:32Z
updated_at: 2026-03-24T15:49:14Z
parent: litmus-49jz
---

Set up the staging infrastructure:

- Create fixtures/candidates/ directory
- Add fixtures/candidates/README.md documenting the review workflow and quality criteria
- Add a REVIEW.md template for candidate assessment
- Optionally: a small script that runs capture on all candidates for quick visual review

## Summary of Changes

Created fixtures/candidates/ staging directory with:
- README.md documenting the review workflow (create → test → parse → assess → promote/discard)
- Quality criteria table (color variety, instant recognition, 80x24, deterministic, self-contained)
- REVIEW.md template for candidate assessment with checkboxes and color coverage notes
- .gitkeep to track the directory in git when empty
