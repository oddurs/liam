---
id: 65
uid: 7170ed34-8b91-4418-ad10-1bc89a48ad5d
title: 'Cloud drivers: ENA, gVNIC and NVMe'
type: feature
status: planned
milestone: v0.4
depends_on:
- 22
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: kernel
effort: m
---

## Problem

Cloud VMs need their NIC and disk drivers. Each one added costs boot time and size.

## Proposal

Build in AWS ENA, Google gVNIC and NVMe, and nothing else from the cloud driver list. Measure what each adds to boot.

## Budget impact

`boot`, `size`.

## Acceptance criteria

- [ ] Boots with networking on AWS Nitro and GCP.
- [ ] The boot cost of each driver is recorded in the kernel config comments.
- [ ] The Firecracker kernel is unaffected. A separate config fragment is fine.
