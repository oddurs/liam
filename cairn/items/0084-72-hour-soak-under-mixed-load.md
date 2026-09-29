---
id: 84
uid: 400658b6-9f2e-4dff-aff6-e395ac8fb68d
title: 72-hour soak under mixed load
type: chore
status: planned
milestone: v1.0
depends_on:
- 44
- 55
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: test
effort: m
---

## Purpose

Leaks and slow degradation only show up over days.

## Approach

Run 72 hours of mixed static, TLS, HTTP/2, proxy and WebSocket load, with reloads every hour and a liamd kill every 6 hours.

## Acceptance criteria

- [ ] RSS grows by less than 5% after the first hour.
- [ ] Open fds at the end equal those at hour one.
- [ ] p99 in the last hour is within 10% of the second hour.
- [ ] The raw data is in `bench/results`.
