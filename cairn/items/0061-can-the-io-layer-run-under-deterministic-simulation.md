---
id: 61
uid: ddb09c71-7bec-4b02-8126-b4665ca1ebb8
title: Can the io layer run under deterministic simulation?
type: spike
status: planned
milestone: v0.3
depends_on:
- 17
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: test
effort: m
---

## Question

Can `liam-io`'s reactor be swapped for a simulated one, with a controlled clock, network and disk, so a failing test replays exactly from a seed?

## Why it blocks

The v1.0 simulation-testing item. The stability promise names it.

## Timebox

Three days.

## Approach

Look at how turmoil and madsim approach it. Try putting the reactor behind a trait at the narrowest seam, and measure what the indirection costs in the release build (it should be zero with generics). Inject one fault: a peer that stalls mid-headers.

## Acceptance criteria

- [ ] A prototype replays one fault deterministically from a seed.
- [ ] The release-build overhead is measured.
- [ ] Closed with `--result`, saying go or no-go.
