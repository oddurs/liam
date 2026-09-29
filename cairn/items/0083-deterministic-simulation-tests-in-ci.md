---
id: 83
uid: b7f17f57-c85c-4a0c-adae-fd93beadf566
title: Deterministic simulation tests in CI
type: chore
status: planned
milestone: v1.0
depends_on:
- 61
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: test
effort: l
---

## Purpose

The stability section promises the event loop runs under simulation with injected faults.

## Approach

Follow the simulation spike's result. Build a seed-driven simulation for the io layer and the HTTP state machines with dropped packets, slow peers, full disks and clock jumps. CI runs a fixed seed set, and a nightly job runs random seeds. A failing seed becomes a bug item that includes the seed.

## Acceptance criteria

- [ ] Fixed seeds run in `scripts/task test`.
- [ ] Any failing seed reproduces locally with one command.
- [ ] If the spike said no-go, this item is dropped with the reason, and the site copy changes to match.
