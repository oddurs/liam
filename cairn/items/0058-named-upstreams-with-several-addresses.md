---
id: 58
uid: 76dffa6f-c21b-4eb7-a828-fb191bb5ce90
title: Named upstreams with several addresses
type: feature
status: planned
milestone: v0.3
depends_on:
- 55
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: proxy
effort: m
---

## Problem

Some apps run as several instances behind one name.

## Proposal

`[upstream.<name>]` takes a list of addresses, uses round-robin, and ejects an address that fails passively for a cooldown period.

## Budget impact

None.

## Acceptance criteria

- [ ] Load spreads evenly across three addresses.
- [ ] A dead address is ejected and later retried.
