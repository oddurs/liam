---
id: 17
uid: de8183b7-1284-461b-960f-81e1cb654c1a
title: Thread-per-core io layer with SO_REUSEPORT listeners
type: feature
status: planned
milestone: v0.1
depends_on:
- 15
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: io
effort: l
budget:
- listen
- no-alloc
---

## Problem

liamd needs one event loop per core with nothing shared between them, or cores wait on each other under load.

## Proposal

`liam-io` spawns one pinned thread per online CPU. Each thread has its own ring and its own `SO_REUSEPORT` listener, or an inherited fd once liam-init owns sockets. It uses multishot accept, runs a task per connection, and keeps a timer wheel per core. Built on the runtime the spike chose.

## Budget impact

`listen`: worker start is on the startup path. `no-alloc`: connection state lives in per-core slabs, allocated up front.

## Acceptance criteria

- [ ] N workers for N CPUs, each pinned. A test reads `Cpus_allowed_list` for each thread.
- [ ] With 4 cores and 1,000 connections, every worker accepts some.
- [ ] On shutdown, workers stop accepting and finish in-flight work.
- [ ] `unsafe` appears only in this crate, and every block has a `SAFETY:` comment.
