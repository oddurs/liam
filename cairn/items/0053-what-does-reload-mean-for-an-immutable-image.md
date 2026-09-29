---
id: 53
uid: a973fb3b-afe7-4560-b649-ed2cfc1f2634
title: What does reload mean for an immutable image?
type: spike
status: planned
milestone: v0.3
depends_on:
- 20
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: config
effort: s
budget:
- zero-drop
---

## Question

The config and site are baked into the image. What changes at runtime that a reload must pick up, and where does it come from? Candidates:

1. Certificates and secrets rotated in mounted files, which ACME renewal needs anyway.
2. `LIAM_*` environment overrides. These cannot change in a running process, so probably not.
3. A new pack and config pushed over the admin socket, for site updates in milliseconds without a reboot.

## Why it blocks

The reload feature and the zero-drop gate's reload case both depend on what reload means.

## Timebox

One day.

## Approach

Write down the operator stories for each candidate: Kubernetes secret rotation, ACME renewal, and a site deploy. Check candidate 3 against the immutability promise and the `zero-drop` decision.

## Acceptance criteria

- [ ] Each candidate is accepted or rejected, with the reason.
- [ ] The reload item's proposal is rewritten to match.
- [ ] Closed with `--result`.
