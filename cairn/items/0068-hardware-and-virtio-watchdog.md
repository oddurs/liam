---
id: 68
uid: 8a95620c-5cfc-4bdd-b2e8-0469ef9a91dd
title: Hardware and virtio watchdog
type: feature
status: planned
milestone: v0.4
depends_on:
- 23
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: ops
effort: m
---

## Problem

A hung machine with no shell can only be fixed by rebooting it, and something has to notice the hang.

## Proposal

liam-init opens `/dev/watchdog` when present and pets it only while liamd's workers report progress. Boot takes milliseconds, so a reboot is a cheap recovery.

## Budget impact

None.

## Acceptance criteria

- [ ] In QEMU with i6300esb, a deliberately wedged liamd causes a reboot within the watchdog timeout.
- [ ] A clean shutdown disarms the watchdog.
