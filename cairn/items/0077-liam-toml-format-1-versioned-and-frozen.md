---
id: 77
uid: 114244c4-be07-4d5f-bcaa-a2dcc1c2f510
title: 'liam.toml format 1: versioned and frozen'
type: feature
status: planned
milestone: v1.0
depends_on:
- 20
- 54
- 62
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: config
effort: m
---

## Problem

A config format people cannot rely on across upgrades is not stable, however fast the server is.

## Proposal

A `format = 1` key is required. The policy is published: within 1.x, keys are only added, never removed or given a new meaning. `liam check` warns on deprecated keys and names the replacement. A pre-1.0 config gets an error naming each key that changed and what it became.

## Budget impact

None.

## Acceptance criteria

- [ ] The compatibility policy is written down, and a test holds every 1.0 key's meaning fixed.
- [ ] Each 0.4 config in the fixtures either loads or gets a per-key upgrade message.
