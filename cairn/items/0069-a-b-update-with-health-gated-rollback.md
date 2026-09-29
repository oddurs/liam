---
id: 69
uid: 96f2e913-06c1-4521-9c71-a77d4bf1604c
title: A/B update with health-gated rollback
type: feature
status: planned
milestone: v0.4
depends_on:
- 63
- 64
- 68
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: deploy
effort: l
---

## Problem

A bad image on a VM must undo itself without a person logging in, because nobody can log in.

## Proposal

The new image is written to the idle slot and booted once with `BootNext`. liam-init confirms the slot once `/healthz` passes N times within T seconds, by making it the permanent boot entry. If there is no confirmation, the watchdog or the timeout reboots into the old slot.

## Budget impact

None.

## Acceptance criteria

- [ ] In QEMU and on AWS, an image whose health check fails returns to the old slot without anyone acting.
- [ ] An image that hangs before liam-init starts also returns, through the watchdog.
- [ ] The slot history is visible in logs after rollback.
