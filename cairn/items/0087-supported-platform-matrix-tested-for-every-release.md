---
id: 87
uid: 47fe5d23-315a-4373-bc96-eaeaecc28f40
title: Supported platform matrix tested for every release
type: chore
status: planned
milestone: v1.0
depends_on:
- 25
- 44
- 64
- 66
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: infra
effort: m
---

## Purpose

"Runs on X" is only true if X is tested for each release.

## Approach

The release workflow boots and smoke-tests every target: OCI on amd64 and arm64, Firecracker on x86_64 and aarch64, QEMU with OVMF, AWS, and GCP. A release fails if any target fails.

## Acceptance criteria

- [ ] The matrix runs for the release candidate, with results attached to the release.
- [ ] The supported-platforms table on the site is generated from this matrix.
