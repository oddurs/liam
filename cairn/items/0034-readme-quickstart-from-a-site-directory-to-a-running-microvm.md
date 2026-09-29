---
id: 34
uid: 66f5663b-11b9-4b03-be5f-0773d0820c2d
title: 'README quickstart: from a site directory to a running microVM'
type: docs
status: planned
milestone: v0.1
depends_on:
- 26
- 28
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: docs
effort: s
---

## Reader and question

Someone with a Linux host with KVM and a built static site asks: how do I get this served by liam?

## Change

A README with host requirements, the install command, a minimal `liam.toml`, `liam build`, `liam run`, the budgets, and a link to the latest gate results.

## Acceptance criteria

- [ ] Someone who did not write it follows it on a clean Ubuntu host and gets their site served.
- [ ] Every command in it is copied from a run against the release.
