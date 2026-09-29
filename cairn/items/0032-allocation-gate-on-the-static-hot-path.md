---
id: 32
uid: cbab5ef1-ce01-48c9-9143-ea2f362df0b1
title: Allocation gate on the static hot path
type: feature
status: planned
milestone: v0.1
depends_on:
- 24
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: bench
effort: s
budget:
- no-alloc
---

## Problem

Allocations creep into hot paths one reasonable-looking change at a time.

## Proposal

An integration test with a counting global allocator warms up, serves 10,000 requests across the fixture site, and asserts zero new allocations. It runs in `scripts/task test`.

## Budget impact

`no-alloc`: this enforces it.

## Acceptance criteria

- [ ] The test passes on the static path.
- [ ] A scratch change that allocates per request fails it.
