---
id: 88
uid: ed25532c-7410-489c-ad6f-fe235264c561
title: HTTP/3 over QUIC
type: feature
status: backlog
milestone: later
depends_on:
- 39
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: http
effort: xl
---

## Problem

HTTP/3 improves latency for clients on lossy or mobile networks. It is also the most expensive protocol to serve well: QUIC runs in userspace, so there is no kTLS or sendfile.

## Proposal

quinn on the liam runtime, using UDP GSO and GRO, with `Alt-Svc` advertisement. Spike the runtime integration and the CPU cost per request first.

## Budget impact

`no-alloc` is unlikely to hold for QUIC. Decide whether the budget excludes it.

## Acceptance criteria

- [ ] Split into a spike and features before it is scheduled.
