---
id: 20
uid: 8ea1679c-263f-48fc-a748-d817600b70ea
title: 'liam.toml v0: parse and validate at build time'
type: feature
status: planned
milestone: v0.1
depends_on:
- 12
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: config
effort: m
---

## Problem

A typo in a config should fail on a laptop, never at boot in production.

## Proposal

Parse with serde and `toml`, keeping spans. v0.1 keys: `[site] root, compress, index, not_found, spa_fallback` and `[limits]`. Unknown keys are errors. Errors print `file:line`, the problem, and a `help:` line. The result is compiled into a binary config stored in the pack, so liamd never parses TOML at boot.

## Budget impact

`boot`: nothing is parsed at startup.

## Acceptance criteria

- [ ] Every key has a test for a valid value and an invalid one.
- [ ] An unknown key's error names it and suggests the nearest valid key.
- [ ] Snapshot tests cover the error output for five common mistakes, including a missing `spa_fallback` file.
- [ ] `liam check` reports the same errors as `liam build`.
