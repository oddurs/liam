---
id: 31
uid: 739582a5-7a3d-4aae-8d87-fb1692d05fda
title: 'CI gate: fail a pull request that misses the boot budget'
type: feature
status: planned
milestone: v0.1
depends_on:
- 13
- 29
- 30
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: bench
effort: m
budget:
- boot
---

## Problem

A budget that is only measured by hand drifts.

## Proposal

A CI job runs the boot harness on the runner the spike chose. It compares p99 to the budget and p50 to main's last result, and fails on either. Thresholds live in one `budgets.toml` that every gate reads. The numbers go in the job summary.

## Budget impact

`boot`: this enforces it.

## Acceptance criteria

- [ ] A pull request that adds a 10 ms sleep to liam-init fails the gate.
- [ ] Every run's numbers appear in the job summary.
- [ ] `budgets.toml` is the only place the thresholds are written.
