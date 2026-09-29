---
id: 67
uid: fef11ed2-540f-4378-a3e0-558965b615b4
title: What does liam deploy do on AWS and GCP?
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

What is the fastest reliable path from a built disk to a running instance on each cloud? Consider:

- image import: EBS direct APIs such as coldsnap, the AWS VM import service, or GCP image create from a tar.gz;
- credentials: the SDK default chains;
- updates: replace the instance, or write the idle slot of a running one.

## Why it blocks

Both deploy targets, and how A/B updates are triggered.

## Timebox

Three days.

## Approach

Time each import path end to end with a 50 MB image. Write down which IAM permissions each needs. Prototype the idle-slot write over the admin socket against replacing the instance.

## Acceptance criteria

- [ ] Timings and required permissions per path and per cloud.
- [ ] The deploy items are rewritten to match.
- [ ] Closed with `--result`.
