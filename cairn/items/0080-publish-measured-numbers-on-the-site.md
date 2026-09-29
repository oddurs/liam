---
id: 80
uid: 634d107f-a892-4e04-888e-05c183b90b6e
title: Publish measured numbers on the site
type: feature
status: planned
milestone: v1.0
depends_on:
- 35
- 79
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: site
effort: s
---

## Problem

The site shows design budgets. At 1.0 it has to show measurements.

## Proposal

At site build time, read the latest gate results for `main` and render the measured p50 and p99 next to each budget, with the date, commit and hardware.

## Budget impact

None.

## Acceptance criteria

- [ ] Every budget tile shows a measured number and where it came from.
- [ ] The "budget, not a measurement" wording is gone wherever a measurement exists.
