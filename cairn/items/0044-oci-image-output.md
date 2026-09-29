---
id: 44
uid: 8a7866bf-4aae-49a7-a1e3-2c870d86b7d8
title: OCI image output
type: feature
status: planned
milestone: v0.2
depends_on:
- 19
- 20
- 23
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: image
effort: m
budget:
- size
---

## Problem

Most people deploy containers. liam has to be a container image before it is anything else to them.

## Proposal

`liam build` writes an OCI image layout tarball for `targets = ["oci"]`. It is built from scratch, with liam-init as the entrypoint, plus liamd and the pack. It has no shell and no libc. The digest is reproducible.

## Budget impact

`size`.

## Acceptance criteria

- [ ] `docker load` and `podman load` accept it, and `docker run -p 8080:8080` serves the site.
- [ ] Two builds of the same input produce the same digest.
- [ ] The image size is printed by `liam build`.
