---
id: 60
uid: 2ec6b564-e1e5-4e26-a9c6-7577f79e5eac
title: 'Zero-drop gate: load through reload, drain and crash'
type: feature
status: planned
milestone: v0.3
depends_on:
- 31
- 45
- 52
- 54
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: bench
effort: m
budget:
- zero-drop
---

## Problem

`zero-drop` is the budget most likely to regress quietly.

## Proposal

The harness drives a fixed-rate load with oha and, in turn, triggers a reload, sends SIGTERM, and sends SIGKILL to liamd. It counts failed requests and refused connections separately, and reads its thresholds from `budgets.toml`.

## Budget impact

`zero-drop`: this enforces it.

## Acceptance criteria

- [ ] The gate runs on every pull request.
- [ ] A scratch change that closes listeners on reload fails it.
