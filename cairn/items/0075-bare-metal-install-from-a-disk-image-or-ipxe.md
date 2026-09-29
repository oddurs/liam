---
id: 75
uid: d8aaaae0-d903-48e8-b5b7-f5445bba87f7
title: Bare-metal install from a disk image or iPXE
type: feature
status: planned
milestone: v0.4
depends_on:
- 64
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: deploy
effort: m
---

## Problem

People with their own servers need a way to put liam on them.

## Proposal

Write the raw disk with `dd` or any imaging tool, or boot the UKI over HTTP through iPXE. Document one worked example.

## Budget impact

None.

## Acceptance criteria

- [ ] One physical x86_64 server boots from disk and serves the site.
- [ ] iPXE boot works in QEMU.
