---
id: 70
uid: e05009ba-52fe-4dca-a929-3d466e4efc19
title: liam deploy --target aws
type: feature
status: planned
milestone: v0.4
depends_on:
- 64
- 67
- 69
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: deploy
effort: l
---

## Problem

Getting a custom image onto EC2 by hand takes a dozen steps. It should take one.

## Proposal

As the deploy spike decided: upload the image, register it, and either launch or update instances. The update goes through the A/B path, and the command waits for the health confirmation before reporting success.

## Budget impact

None.

## Acceptance criteria

- [ ] A fresh AWS account with the documented IAM policy deploys the example site in one command.
- [ ] A deploy whose `/healthz` fails rolls back and reports it.
- [ ] Credentials come only from the standard AWS chain.
