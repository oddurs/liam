---
id: 23
uid: afa44e22-3d86-42a8-a9be-9ba971770169
title: 'liam-init: PID 1 in the microVM'
type: feature
status: planned
milestone: v0.1
depends_on:
- 12
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: init
effort: m
budget:
- boot
---

## Problem

Something has to be PID 1. systemd costs hundreds of milliseconds and a large attack surface.

## Proposal

A small static Rust binary that:

- mounts `/proc`, `/sys`, `/dev` (devtmpfs) and a tmpfs `/tmp`;
- applies sysctls (`somaxconn`, TCP buffers, `file-max`);
- seeds the RNG;
- starts liamd as a child, reaps zombies, and restarts liamd with backoff if it exits;
- powers off cleanly on ctrl-alt-del.

## Budget impact

`boot`, `size`.

## Acceptance criteria

- [ ] It boots in Firecracker and starts liamd.
- [ ] A killed liamd is restarted within 1 ms, shown by log timestamps.
- [ ] A test shows an orphaned process is reaped.
- [ ] Firecracker's `SendCtrlAltDel` powers the VM off cleanly.
- [ ] The stripped binary is under 300 KB.
