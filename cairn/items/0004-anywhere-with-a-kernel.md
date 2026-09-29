---
id: 4
uid: e365e3c3-abcc-4d2a-898b-0d3a897f6c85
key: v0.4
title: Anywhere with a kernel
type: milestone
status: planned
depends_on:
- 3
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

Run liam anywhere with a kernel: a UEFI image on AWS, GCP and bare metal, on x86_64 and aarch64, updated through A/B slots with automatic rollback, and with ACME certificates.

## Release gate

- [ ] `liam deploy --target aws` puts a site on an EC2 instance, and a bad update rolls itself back.
- [ ] The same site boots on GCP, in QEMU with OVMF, and on one physical server.
- [ ] aarch64 images boot in Firecracker and on Graviton.

## Explicitly not in this milestone

- Other clouds beyond AWS and GCP (later).
