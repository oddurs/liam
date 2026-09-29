---
id: 71
uid: 637a9c16-fd81-4f29-a40d-752b4975aa09
title: liam deploy --target gcp
type: feature
status: planned
milestone: v0.4
depends_on:
- 64
- 67
- 69
created: 2026-09-28
updated: 2026-09-28
priority: p1
area: deploy
effort: l
---

## Problem

The same as AWS, for Google Cloud.

## Proposal

The GCP path from the deploy spike: image create, instance create or update, and the same A/B and health flow.

## Budget impact

None.

## Acceptance criteria

- [ ] One command deploys the example site to GCP.
- [ ] A failing health check rolls back.
