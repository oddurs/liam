---
id: 47
uid: cf6560be-0c00-407b-9b7f-518143b19e47
title: CI gates for exec-to-listen and image size
type: feature
status: planned
milestone: v0.2
depends_on:
- 31
- 44
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: bench
effort: m
budget:
- listen
- size
---

## Problem

The `listen` and `size` budgets need the same enforcement the `boot` budget has.

## Proposal

A harness runs the image with runc directly and times from execve of liam-init to the first HTTP response, 100 times. A size check reads the compressed layer total from the manifest of the empty-site fixture. Both read their thresholds from `budgets.toml`.

## Budget impact

`listen`, `size`: this enforces them.

## Acceptance criteria

- [ ] A pull request that adds 200 KB to liamd fails the size gate.
- [ ] A 10 ms sleep before listen fails the listen gate.
- [ ] Both report their numbers in the job summary.
