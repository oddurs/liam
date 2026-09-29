---
id: 8
uid: 14f6d380-3102-4272-b794-b7c9cc87af4e
key: listen
title: Exec to listening under 5 ms
type: budget
status: planned
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

## Promise

As a container on an existing host kernel, liam accepts connections within 5 ms of liam-init being executed.

## How it is measured

The harness runs the container 100 times with runc directly, so the Docker daemon's overhead is not counted. It times from execve of liam-init to the first successful TCP connect and HTTP response, and reports p50 and p99.

## Gate

p99 is under 5 ms.

## Measured

Not yet measured.
