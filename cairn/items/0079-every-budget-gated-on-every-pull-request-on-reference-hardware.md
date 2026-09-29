---
id: 79
uid: 6175291f-21bb-4e76-89f3-416d928e2884
title: Every budget gated on every pull request, on reference hardware
type: chore
status: planned
milestone: v1.0
depends_on:
- 31
- 32
- 47
- 60
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: bench
effort: m
budget:
- boot
- listen
- size
- zero-drop
- no-alloc
---

## Purpose

The promise is that every published number is enforced. By v1.0, none may be measured only by hand.

## Approach

Run all five gates on the reference host chosen by the gate spike, for x86_64 and aarch64. Keep a history of results per commit on `main`.

## Acceptance criteria

- [ ] Five gates on two architectures run on every pull request.
- [ ] A history of results for `main` is kept and readable, for example as JSON published with the site.
