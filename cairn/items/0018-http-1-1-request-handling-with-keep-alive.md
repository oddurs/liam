---
id: 18
uid: e351f2f3-3bfc-4965-a803-c6b206f44a5f
title: HTTP/1.1 request handling with keep-alive
type: feature
status: planned
milestone: v0.1
depends_on:
- 17
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: http
effort: l
budget:
- no-alloc
---

## Problem

liamd has to speak HTTP/1.1 correctly and without per-request allocation.

## Proposal

Parse with `httparse` into fixed per-connection buffers. Support GET and HEAD with keep-alive. Tolerate pipelined requests by answering them in order. Refuse request bodies in v0.1 with 411 or 413, because v0.1 serves static files only. Cache the `Date` header once per second per core. Honour `Connection: close`.

## Budget impact

`no-alloc`: this is the hot path.

## Acceptance criteria

- [ ] A table-driven test of at least 30 malformed requests, each answered with 400 and a closed connection.
- [ ] HEAD returns the same headers as GET with no body.
- [ ] 1,000 sequential requests on one socket reuse the connection.
- [ ] curl, wrk and oha all complete against it without errors.
