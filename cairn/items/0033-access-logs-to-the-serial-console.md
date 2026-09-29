---
id: 33
uid: 274c9868-bdbd-42d4-9928-95bddcd19f18
title: Access logs to the serial console
type: feature
status: planned
milestone: v0.1
depends_on:
- 18
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: ops
effort: m
---

## Problem

With no shell, logs are the only window into a running instance.

## Proposal

One JSON line per request, with ts, method, path, status, bytes, duration in µs, and core. Each core writes to its own ring buffer, and a low-priority thread flushes them to stdout or the serial console. When a ring is full, lines are dropped and counted rather than blocking the request.

## Budget impact

`no-alloc`: logging uses preallocated rings.

## Acceptance criteria

- [ ] Every request produces one JSON line.
- [ ] A saturated console raises request p99 by less than 5%, measured.
- [ ] The count of dropped lines is logged every 10 seconds when non-zero.
