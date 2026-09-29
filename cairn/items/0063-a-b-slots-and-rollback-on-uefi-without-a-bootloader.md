---
id: 63
uid: bfd0e33e-6dcb-4cd7-802c-9cf80ff95179
title: A/B slots and rollback on UEFI without a bootloader
type: spike
status: planned
milestone: v0.4
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: deploy
effort: m
---

## Question

Can a unified kernel image that firmware boots directly do A/B updates with automatic rollback, using EFI `BootNext`, `BootOrder` and a health confirmation? Or does it need a bootloader with boot counting, such as systemd-boot?

## Why it blocks

The UKI layout and A/B updates both depend on the answer. Firmware behaviour varies by vendor, so this is the likeliest place for v0.4 to slip.

## Timebox

Four days.

## Approach

Try it in QEMU with OVMF, on AWS Nitro, on GCP, and on one physical server. For each, check that `BootNext` survives exactly one boot, that EFI variables are writable from the running image, and what happens when the new slot hangs before confirming.

## Acceptance criteria

- [ ] A table of what works on each of the four platforms.
- [ ] A chosen mechanism, with its failure modes written down.
- [ ] Closed with `--result`.
