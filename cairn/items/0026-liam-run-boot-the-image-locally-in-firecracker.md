---
id: 26
uid: 1b5d6586-1157-4e97-81a3-e33ee0058a12
title: 'liam run: boot the image locally in Firecracker'
type: feature
status: planned
milestone: v0.1
depends_on:
- 25
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: cli
effort: m
---

## Problem

The first thing a new user does is run it. That has to work, or say exactly why it didn't.

## Proposal

`liam run` finds `firecracker` on PATH or through `--firecracker`, sets up a tap device (or uses one given with `--tap`), boots the `.fc` image, streams the serial console to the terminal, prints the URL, and shuts the VM down cleanly on Ctrl-C.

## Budget impact

None.

## Acceptance criteria

- [ ] On Linux with `/dev/kvm`, `liam run` in the example directory serves the site at the printed URL.
- [ ] A missing `/dev/kvm`, `firecracker` binary or tap permission each give an error that names the fix.
- [ ] Ctrl-C leaves no Firecracker process and no tap device that liam created.
