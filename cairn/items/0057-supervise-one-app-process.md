---
id: 57
uid: 45d255b6-2cc7-4776-8ca5-4846e9cc39cd
title: Supervise one app process
type: feature
status: planned
milestone: v0.3
depends_on:
- 23
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: init
effort: m
---

## Problem

Most dynamic sites are one app server. Running a second container just to supervise it undoes liam's point.

## Proposal

The `[app]` section takes `exec`, `args`, `env`, `user`, `workdir` and `port`, plus `include` paths that `liam build` copies into the image. liam-init:

- starts the app and routes to it only once a TCP connect to its port succeeds;
- restarts it with exponential backoff if it exits;
- tags its stdout and stderr lines as `source=app` in the logs;
- forwards SIGTERM with a grace period.

The app must be a self-contained executable, because liam provides no language runtimes.

## Budget impact

None. The app's own startup is outside the budgets.

## Acceptance criteria

- [ ] A static Go binary and a bundled Node single-executable app each run and serve through a proxy route.
- [ ] An app that crashes in a loop backs off to a capped interval, and the reason appears in the logs.
- [ ] Requests to the app before it is ready get 503 with `Retry-After`, not a connection error.
