---
# litmus-z7p4
title: Integration test skeleton for essential features
status: draft
type: task
priority: normal
created_at: 2026-03-26T14:51:12Z
updated_at: 2026-03-26T14:51:12Z
---

Set up a skeleton for integration/smoke tests that verify essential UI features don't regress across refactors. Start with contrast issue visualization as the first tracked feature.

## Requirements

- [ ] Define test infrastructure (framework, how tests run, what they verify)
- [ ] Add first test case: contrast issues are detected and displayed for a known theme with known violations
- [ ] Document pattern for adding new essential feature tests

## Notes

This is a draft — details to be refined later. The goal is to have a lightweight safety net for features that have regressed before (contrast display being the first example).
