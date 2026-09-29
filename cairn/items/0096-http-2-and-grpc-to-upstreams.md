---
id: 96
uid: 7a4d74c2-a827-4c29-a0c6-8263fe01f108
title: HTTP/2 and gRPC to upstreams
type: feature
status: backlog
milestone: later
depends_on:
- 39
- 55
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: proxy
effort: l
---

## Problem

gRPC services need HTTP/2 end to end.

## Proposal

HTTP/2 upstream connections with trailers passed through, and h2c for plaintext upstreams.

## Budget impact

None.

## Acceptance criteria

- [ ] Split before it is scheduled.
