---
id: 74
uid: 02462e81-ebdd-428c-bfa3-56262ef47a05
title: Boot measurements on cloud VMs and bare metal
type: feature
status: planned
milestone: v0.4
depends_on:
- 64
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: bench
effort: m
budget:
- boot
---

## Problem

Firmware takes seconds, and the site says so. What liam controls after firmware handoff should still be measured and published.

## Proposal

The harness reads kernel and liam-init timestamps from the serial console on AWS, GCP and one physical server. It reports firmware handoff → first byte separately from total wall time.

## Budget impact

`boot`: informational outside Firecracker.

## Acceptance criteria

- [ ] Numbers for three platforms, with instance types.
- [ ] Results are in `bench/results`.
