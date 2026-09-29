---
id: 22
uid: f75c295a-1756-4a67-90b3-037003981df9
title: Kernel config and reproducible kernel build
type: feature
status: planned
milestone: v0.1
depends_on:
- 14
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: kernel
effort: l
budget:
- boot
---

## Problem

The kernel is a third of the image and most of the boot time. It has to be small, fast and rebuildable bit for bit.

## Proposal

`kernel/` holds the pinned LTS version, the config fragment from the boot spike, and a build script that runs in a container. It outputs an uncompressed `vmlinux` for Firecracker, with a built-in initramfs containing liam-init and liamd, which are fixed per liam release. Networking uses the kernel's `ip=` autoconfiguration, which costs no userspace time.

## Budget impact

`boot`: this is most of it.

## Acceptance criteria

- [ ] Two clean builds produce the same sha256, using `SOURCE_DATE_EPOCH` and `KBUILD_BUILD_*`.
- [ ] `CONFIG_MODULES` is off.
- [ ] The config fragment comments every non-obvious option.
- [ ] It boots in Firecracker to liam-init, with the size of `vmlinux` recorded.
