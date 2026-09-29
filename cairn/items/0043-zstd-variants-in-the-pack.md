---
id: 43
uid: 2e26f238-7bdd-4854-8d58-fa3f623b2dd1
title: zstd variants in the pack
type: feature
status: planned
milestone: v0.2
depends_on:
- 19
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: pack
effort: s
---

## Problem

Current browsers accept zstd, which decompresses faster than Brotli for similar size.

## Proposal

Add a zstd variant at build time, at level 19 by default. Order negotiation as br, zstd, gzip, unless q-values say otherwise.

## Budget impact

None.

## Acceptance criteria

- [ ] A zstd variant round-trips in the pack tests.
- [ ] Negotiation tests include zstd.
