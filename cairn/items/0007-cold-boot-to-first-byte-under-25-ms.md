---
id: 7
uid: d1c70b77-a528-4b48-83c4-930d2cf036e5
key: boot
title: Cold boot to first byte under 25 ms
type: budget
status: planned
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

## Promise

A Firecracker microVM running liam answers its first HTTP request within 25 ms of the VMM process starting.

## How it is measured

The boot harness starts Firecracker 100 times with the example site on the reference host. It times from VMM process start to the first successful HTTP response, and uses Firecracker's boot-timer device to time the guest-side phases. It reports p50 and p99.

## Gate

p99 is under 25 ms, and p50 has not regressed more than 10% against main.

## Measured

Not yet measured.
