---
id: 46
uid: 69dc54b9-f2bc-4209-87e7-85ba5f0ee7bb
title: liam run on macOS through Docker or Podman
type: feature
status: planned
milestone: v0.2
depends_on:
- 44
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: cli
effort: s
---

## Problem

Many people build sites on macOS, where Firecracker does not run.

## Proposal

On macOS, or with `--oci`, `liam run` loads the OCI image into Docker or Podman, whichever it finds, runs it, and prints the URL.

## Budget impact

None.

## Acceptance criteria

- [ ] `liam run` on macOS with Docker Desktop serves the example site.
- [ ] When neither Docker nor Podman is found, the error says which to install.
