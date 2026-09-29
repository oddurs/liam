---
id: 9
uid: 694b9e83-26fb-46e3-8db8-5be84fcfb890
key: size
title: Base container image under 6 MB
type: budget
status: planned
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

## Promise

The OCI image for an empty site is under 6 MB: liam-init, liamd and the compiled config, statically linked.

## How it is measured

`liam build` on the empty-site fixture, reporting the total compressed layer size in the image manifest.

## Gate

The total is under 6 MB, and no single pull request grows it by more than 100 KB without a note explaining why.

## Measured

Not yet measured.
