---
id: 59
uid: df2597c2-e26a-4c19-924a-a541c68ef657
title: Prometheus metrics on an admin port
type: feature
status: planned
milestone: v0.3
depends_on:
- 17
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: ops
effort: m
---

## Problem

Operators need rates, errors and latency without a shell.

## Proposal

Per-core counters are summed when scraped:

- requests by status class;
- a latency histogram with fixed buckets;
- open connections and TLS handshakes;
- dropped log lines;
- boot phase timings.

They are served on a separate admin port that is off by default and never on a public listener.

## Budget impact

`no-alloc`: counters are per-core atomics with no allocation.

## Acceptance criteria

- [ ] The metrics follow Prometheus naming conventions and pass `promtool check metrics`.
- [ ] Scraping under load does not move request p99, measured.
- [ ] The admin port cannot be set to a public listener's port.
