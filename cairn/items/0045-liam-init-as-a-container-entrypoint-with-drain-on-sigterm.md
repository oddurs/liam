---
id: 45
uid: 3a00598d-393d-41a6-903a-7b3fff160015
title: liam-init as a container entrypoint, with drain on SIGTERM
type: feature
status: planned
milestone: v0.2
depends_on:
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

In a container, liam-init cannot mount or set most sysctls. Kubernetes sends SIGTERM and expects a drain before the grace period ends.

## Proposal

liam-init detects that it is in a container and skips privileged setup. On SIGTERM, liamd stops accepting, finishes in-flight requests, and exits before a configurable deadline (default 25 s, under Kubernetes' 30 s). It runs as a non-root user on a read-only root filesystem with no capabilities, listening on 8080 by default.

## Budget impact

`zero-drop` for drain.

## Acceptance criteria

- [ ] `docker stop` under load fails no in-flight requests and exits 0.
- [ ] It runs with `--read-only --cap-drop=ALL --user 65532`.
- [ ] Requests still in flight at the deadline are counted in the exit log.
