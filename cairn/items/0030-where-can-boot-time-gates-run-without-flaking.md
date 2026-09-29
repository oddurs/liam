---
id: 30
uid: 94d7f60f-71c6-4234-91ef-b3b06bf10a54
title: Where can boot-time gates run without flaking?
type: spike
status: planned
milestone: v0.1
depends_on:
- 29
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: bench
effort: s
budget:
- boot
---

## Question

Do GitHub-hosted Linux runners expose `/dev/kvm` with low enough timing variance for a 25 ms gate, or does the gate need a dedicated self-hosted machine?

## Why it blocks

A flaky gate gets ignored, and then the budget is decoration.

## Timebox

One day, plus runs spread over 24 hours.

## Approach

Run the boot harness 20 times on hosted runners at different times of day, and on one dedicated bare-metal host. Compare the spread of p50 and p99. Price the dedicated option.

## Acceptance criteria

- [ ] The variance of both options, with raw numbers.
- [ ] A decision on where gates run, with the monthly cost.
- [ ] Closed with `--result`.
