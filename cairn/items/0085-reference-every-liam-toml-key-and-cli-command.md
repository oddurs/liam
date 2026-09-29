---
id: 85
uid: 49de789e-2389-43e2-83d8-743fb54d767d
title: 'Reference: every liam.toml key and CLI command'
type: docs
status: planned
milestone: v1.0
depends_on:
- 77
- 78
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: docs
effort: m
---

## Reader and question

Someone configuring liam asks what a key does, what its default is, and what happens at its limit.

## Change

A generated reference that the config schema and clap produce, so it cannot drift, with hand-written explanations for each section.

## Acceptance criteria

- [ ] Every key and flag appears, with its type, default and an example.
- [ ] CI fails if the generated reference is out of date.
