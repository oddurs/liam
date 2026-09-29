---
id: 40
uid: 43a6afc8-8a09-425f-a8b3-e81a383974bf
title: Host routing, redirects and response headers
type: feature
status: planned
milestone: v0.2
depends_on:
- 20
- 24
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: router
effort: m
budget:
- no-alloc
---

## Problem

One image serves several domains, needs canonical redirects, and must set caching and security headers per route.

## Proposal

`liam build` compiles the routes. Hosts go into a perfect hash and paths into a radix tree. Supported:

- redirect rules: http → https, www → apex, and arbitrary 301 and 308;
- per-route `headers`;
- `cache = "immutable"`, which sets `Cache-Control`;
- HSTS.

An unknown `Host` gets 421 Misdirected Request.

## Budget impact

`no-alloc`: route lookup must not allocate.

## Acceptance criteria

- [ ] A table test covers routing across three hosts, with redirects and header rules.
- [ ] Route lookup is allocation-free, checked by the allocation gate.
- [ ] Overlapping or unreachable routes fail `liam build`, with the conflicting lines named.
