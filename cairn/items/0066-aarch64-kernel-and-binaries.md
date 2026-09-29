---
id: 66
uid: 1ec00da7-fefc-48a6-9cde-68271e76e8de
title: aarch64 kernel and binaries
type: feature
status: planned
milestone: v0.4
depends_on:
- 22
- 28
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: kernel
effort: l
---

## Problem

Graviton and Ampere machines are often the cheapest way to serve.

## Proposal

Cross-build the kernel, liam-init and liamd for aarch64. Boot on Firecracker on Graviton bare metal, and as a UKI on Graviton EC2. Releases carry both architectures, and OCI output becomes a multi-arch index.

## Budget impact

`boot`, `size`, `listen`: every gate runs on aarch64 too.

## Acceptance criteria

- [ ] Firecracker on a Graviton metal instance serves the example site.
- [ ] A Graviton EC2 instance boots the UKI.
- [ ] The OCI index holds amd64 and arm64, and `docker run` picks the right one.
- [ ] The budget gates run on aarch64.
