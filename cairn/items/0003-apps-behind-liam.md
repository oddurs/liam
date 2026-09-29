---
id: 3
uid: 78ce6f12-c2a7-4d5b-b34d-eecae4a7018e
key: v0.3
title: Apps behind liam
type: milestone
status: planned
depends_on:
- 2
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

Put a dynamic app behind liam. It reverse-proxies to upstreams or to one process it supervises, and it reloads, restarts and drains without failing a request.

## Release gate

- [ ] A Go or Node app built as a self-contained executable runs under liam-init and is proxied, including WebSockets.
- [ ] The zero-drop gate passes: no failed requests across reload and drain, no refused connections across a liamd crash.
- [ ] Metrics are scrapeable from the admin port.

## Explicitly not in this milestone

- Cloud VM and bare-metal images (v0.4).
- Proxy response caching (later).
