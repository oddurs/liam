---
id: 13
uid: 260e57f1-6e39-40c6-8479-549fb0dd9c25
title: 'CI: format, lint and test every pull request'
type: chore
status: planned
milestone: v0.1
depends_on:
- 12
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: infra
effort: s
---

## Purpose

Pull requests become the only way `main` moves, so the checks have to run on them.

## Approach

A GitHub Actions workflow that runs `scripts/task check` on pull requests and on `main`, with cargo caching. CI knows only the task verbs, so it cannot drift from local runs.

## Acceptance criteria

- [ ] The workflow runs `scripts/task check` and nothing else that duplicates it.
- [ ] A pull request with a clippy warning fails CI.
- [ ] A cached run finishes in under 5 minutes.
