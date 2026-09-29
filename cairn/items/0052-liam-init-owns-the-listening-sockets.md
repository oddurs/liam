---
id: 52
uid: 950358fd-2d10-4598-ad9f-f44679193ecf
title: liam-init owns the listening sockets
type: feature
status: planned
milestone: v0.3
depends_on:
- 17
- 23
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: init
effort: m
budget:
- zero-drop
---

## Problem

When liamd exits, its listening sockets close, and new connections are refused until it is back.

## Proposal

liam-init binds every listener with `SO_REUSEPORT`, one per core. It passes them to liamd by inheritance, using `LISTEN_FDS`-style environment variables, and keeps its own copies open. While liamd restarts, the kernel queues new connections in the accept backlog.

## Budget impact

`zero-drop` for a crash.

## Acceptance criteria

- [ ] `kill -9` of liamd under a steady 10k req/s load produces zero refused connections.
- [ ] A test shows the backlog is sized from `somaxconn` and a config key.
- [ ] liamd started without inherited fds still binds its own, for development.
