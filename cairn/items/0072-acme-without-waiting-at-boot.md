---
id: 72
uid: f4d28bd8-dd3a-4537-ae96-1368e7c0fd35
title: ACME without waiting at boot
type: spike
status: planned
milestone: v0.4
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: tls
effort: s
---

## Question

Where are ACME certificates issued, and where are they kept, in an image with no writable persistent storage? Options:

- TLS-ALPN-01 answered by liamd itself;
- DNS-01 through a provider API at deploy time;
- a cache on a small writable partition, or in the cloud's secret store.

Boot must never wait on the network.

## Why it blocks

The ACME feature.

## Timebox

Two days.

## Approach

Walk through first deploy, renewal, and cold boot after renewal for each option, with Let's Encrypt staging.

## Acceptance criteria

- [ ] The chosen issuance and storage path, and why the others were rejected.
- [ ] Closed with `--result`.
