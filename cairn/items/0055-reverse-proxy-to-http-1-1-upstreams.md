---
id: 55
uid: 2d4b76ab-d44b-4fc9-9060-b3488e479a5a
title: Reverse proxy to HTTP/1.1 upstreams
type: feature
status: planned
milestone: v0.3
depends_on:
- 18
- 40
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: proxy
effort: l
---

## Problem

Most sites have an API or an app server behind them.

## Proposal

`proxy = "<upstream>"` on a route forwards the request.

- Each core keeps its own keep-alive pool per upstream.
- Connect and response timeouts are separate.
- A request is retried only when it is idempotent and nothing has been sent upstream yet.
- `X-Forwarded-For`, `-Proto` and `-Host` are set, and `Host` is preserved by default.
- Bodies stream in both directions with backpressure, capped by `body_bytes`.

## Budget impact

None directly. Proxy requests are not on the static path.

## Acceptance criteria

- [ ] An upstream that dies mid-response gives the client a clean 502 without leaking the connection.
- [ ] A 1 GB upload streams through with bounded memory, measured.
- [ ] Retry rules are tested for GET, for POST, and after partial send.
- [ ] Pooled connections are reused, shown by the upstream's connection count under load.
