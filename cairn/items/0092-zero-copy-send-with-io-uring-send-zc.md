---
id: 92
uid: 4644c3fe-de2c-425b-9550-195d1eeb9588
title: Zero-copy send with io_uring SEND_ZC
type: feature
status: backlog
milestone: later
depends_on:
- 17
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: io
effort: m
budget:
- no-alloc
---

## Problem

Large proxied responses and non-kTLS paths still copy.

## Proposal

Use `IORING_OP_SEND_ZC` where the kernel supports it, and measure where it beats plain send. It usually wins only above a size threshold.

## Budget impact

`no-alloc`.

## Acceptance criteria

- [ ] Benchmark showing the crossover size.
