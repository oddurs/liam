---
id: 25
uid: b526be14-0333-4fe1-96b2-9313f3499725
title: 'liam build: assemble a Firecracker image'
type: feature
status: planned
milestone: v0.1
depends_on:
- 16
- 19
- 20
- 21
- 22
- 23
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: image
effort: m
budget:
- boot
---

## Problem

A user has a directory and a `liam.toml`, and needs something Firecracker can boot.

## Proposal

`liam build` validates the config, builds the pack, and writes `<name>.fc/`, which contains:

- the release's `vmlinux`;
- the site pack, laid out as the "where does the site live" spike decided;
- a Firecracker config JSON with the kernel command line, memory and vCPU count.

## Budget impact

`boot`.

## Acceptance criteria

- [ ] `liam build` in the example directory produces a bootable `.fc` directory.
- [ ] Two builds of the same input are byte-identical.
- [ ] The build prints what it produced and each part's size.
