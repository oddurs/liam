---
id: 56
uid: 450d1708-7919-4385-b518-f4f30879205f
title: WebSocket and upgrade passthrough
type: feature
status: planned
milestone: v0.3
depends_on:
- 55
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: proxy
effort: m
---

## Problem

Apps behind a proxy expect WebSockets to work.

## Proposal

On `Upgrade: websocket` for a proxied route, forward the handshake. After 101 Switching Protocols, splice bytes in both directions until either side closes, with an idle timeout.

## Budget impact

None.

## Acceptance criteria

- [ ] An echo WebSocket app behind liam passes the Autobahn client suite's framing tests.
- [ ] An idle upgraded connection closes at the configured timeout.
