---
id: 41
uid: acda9d48-eca9-40f4-be1b-e8526a72fe22
title: Bound every phase of a request
type: feature
status: planned
milestone: v0.2
depends_on:
- 18
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: http
effort: m
---

## Problem

Facing the internet, one slow or hostile client must not hold a core's resources.

## Proposal

Configurable in `[limits]`, all with defaults:

- header bytes (16 KiB), header count, and request-line length;
- timeouts for header read (10 s), keep-alive idle (60 s) and write stall (30 s);
- maximum connections per worker and per client IP.

Exceeding a limit returns 431 or 408, or closes the connection.

## Budget impact

None.

## Acceptance criteria

- [ ] A slowloris client sending one byte per second is closed at the header timeout.
- [ ] Oversized headers get 431.
- [ ] Every limit has a test and a default written in the config schema.
- [ ] Connections over the per-IP cap are closed immediately without affecting other clients.
