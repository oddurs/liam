---
id: 29
uid: 16d02226-9b8a-45e4-a360-4512e194dc91
title: Boot benchmark harness
type: feature
status: planned
milestone: v0.1
depends_on:
- 25
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: bench
effort: m
budget:
- boot
---

## Problem

The boot budget means nothing without a measurement people can rerun.

## Proposal

`bench/boot` starts Firecracker N times with the example site. It times from VMM process start to the first successful HTTP response, using a tight connect-retry loop, and reads the guest-side phases from the boot-timer device. It writes JSON with p50, p99, max, and host metadata (CPU, kernel, Firecracker version).

## Budget impact

`boot`: this is how it is measured.

## Acceptance criteria

- [ ] A 100-run report includes p50, p99 and host metadata.
- [ ] Across 5 invocations on the reference host, p50 varies by less than 10%.
- [ ] The JSON schema is described in the harness's `--help`.
