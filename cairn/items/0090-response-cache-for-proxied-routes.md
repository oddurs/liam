---
id: 90
uid: a1576848-de4f-4510-b692-e8d26ad23b81
title: Response cache for proxied routes
type: feature
status: backlog
milestone: later
depends_on:
- 55
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: proxy
effort: l
---

## Problem

Many apps behind liam produce cacheable responses.

## Proposal

A per-core in-memory cache that honours `Cache-Control` and `Vary`, with a size bound and a purge on the admin socket.

## Budget impact

None.

## Acceptance criteria

- [ ] Split before it is scheduled.
