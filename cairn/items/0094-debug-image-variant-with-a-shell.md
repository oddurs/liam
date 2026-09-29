---
id: 94
uid: 61734e69-e7b0-427b-b060-934ba46ec4bf
title: Debug image variant with a shell
type: feature
status: backlog
milestone: later
created: 2026-09-28
updated: 2026-09-28
priority: p3
area: image
effort: m
---

## Problem

Some failures are hard to diagnose from logs and metrics alone.

## Proposal

`liam build --debug` adds a static busybox and a serial-console shell. It is marked in its name, its metrics and its boot banner, and refuses the production targets unless forced.

## Budget impact

None. It is never the gated image.

## Acceptance criteria

- [ ] Split before it is scheduled.
