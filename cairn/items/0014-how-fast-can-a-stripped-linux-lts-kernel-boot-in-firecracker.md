---
id: 14
uid: 28fe831a-9063-4ece-a6ed-ff6ab292e349
title: How fast can a stripped Linux LTS kernel boot in Firecracker?
type: spike
status: planned
milestone: v0.1
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: kernel
effort: m
budget:
- boot
---

## Question

What is the floor for Firecracker VMM start → kernel → `/init` with a minimal config of the current Linux LTS kernel, and does it leave room for 25 ms to first byte?

## Why it blocks

The boot budget and the kernel config both rest on this number. If the floor is 22 ms, the budget or the approach has to change before anything is built on it.

## Timebox

Two days.

## Approach

1. Start from Firecracker's recommended microVM guest config on the latest LTS. Strip it: no modules, no PCI, virtio-mmio only, uncompressed `vmlinux`, and a built-in initramfs whose static `/init` writes to Firecracker's boot-timer device and powers off.
2. Boot it 100 times on one known host. Record p50 and p99 for VMM start → guest kernel start → `/init` → poweroff.
3. Try each separately: `quiet`, `random.trust_cpu=on`, `tsc=reliable`, removing individual initcalls (use `initcall_debug` to rank them), and LZ4 against uncompressed.
4. Record which options cost the most.

## Acceptance criteria

- [ ] A table of p50 and p99 per configuration, with the host CPU, host kernel and Firecracker version.
- [ ] The winning config fragment is committed under `kernel/`.
- [ ] The `boot` budget is confirmed or revised, with the reason noted on it.
- [ ] Closed with `--result` stating the floor in milliseconds.
