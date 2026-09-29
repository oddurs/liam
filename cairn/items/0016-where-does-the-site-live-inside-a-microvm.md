---
id: 16
uid: 2b5d136d-4a50-4332-8093-d7778866ab9f
title: Where does the site live inside a microVM?
type: spike
status: planned
milestone: v0.1
depends_on:
- 14
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: image
effort: s
budget:
- boot
---

## Question

Should the site pack ride in the initramfs, which the kernel copies into RAM at boot, or on a read-only virtio-blk device that liamd maps and the page cache fills lazily?

## Why it blocks

Unpacking an initramfs costs time in proportion to its size, which puts a large site directly against the boot budget. The answer shapes the image layout and `liam build`.

## Timebox

One day.

## Approach

Measure boot to first byte and the latency of the first cold request with 1 MB, 40 MB and 200 MB packs, both ways, on the spike's kernel.

## Acceptance criteria

- [ ] Numbers for both approaches at all three sizes.
- [ ] The image layout item is updated to match.
- [ ] Closed with `--result` giving the choice and the size where the two cross over.
