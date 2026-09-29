---
id: 27
uid: 2c87c0ab-d9a9-405c-a9ca-1d83f4954fa4
title: Ship a prebuilt kernel with each release
type: feature
status: planned
milestone: v0.1
depends_on:
- 22
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: infra
effort: m
---

## Problem

Nobody should have to compile Linux to try liam.

## Proposal

The release workflow builds the `vmlinux` with liam-init and liamd for each target and uploads it with its sha256. `liam build` downloads the version matching the CLI into a cache and verifies it. `--kernel` points at a custom kernel instead.

## Budget impact

None.

## Acceptance criteria

- [ ] On a fresh machine, `liam build` fetches and verifies the kernel.
- [ ] A checksum mismatch aborts with an error, and the bad file is removed from the cache.
- [ ] With a warm cache, `liam build` works offline.
