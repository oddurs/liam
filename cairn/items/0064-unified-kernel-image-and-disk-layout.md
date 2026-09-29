---
id: 64
uid: 9cde3b56-9d90-495c-93ab-2ab2e6a9b221
title: Unified kernel image and disk layout
type: feature
status: planned
milestone: v0.4
depends_on:
- 22
- 63
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: image
effort: l
budget:
- boot
---

## Problem

Cloud VMs and physical servers boot from disks through UEFI, not from a VMM's direct kernel boot.

## Proposal

`liam build` for `targets = ["aws", "gcp", "metal"]` produces:

- a `.efi` UKI containing the kernel, initramfs and command line;
- a GPT raw disk with an EFI system partition holding slots A and B, and a read-only pack partition per slot.

## Budget impact

`boot`: everything after firmware handoff is held to the same phases as Firecracker.

## Acceptance criteria

- [ ] The disk boots in QEMU with OVMF, on AWS x86_64, and on GCP.
- [ ] Two builds of the same input produce identical images.
- [ ] Time from firmware handoff to first byte is logged from kernel timestamps.
