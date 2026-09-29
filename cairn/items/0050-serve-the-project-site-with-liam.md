---
id: 50
uid: 20843469-eaa7-4c87-a49a-485a7982d67d
title: Serve the project site with liam
type: feature
status: planned
milestone: v0.2
depends_on:
- 35
- 38
- 44
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: site
effort: s
---

## Problem

The best evidence that liam works is the project's own site running on it.

## Proposal

Build `site/` with liam into an OCI image and deploy it wherever is cheapest.

## Budget impact

None.

## Acceptance criteria

- [ ] The public site is served by liam over HTTPS and HTTP/2.
- [ ] Its footer says which liam version serves it.
