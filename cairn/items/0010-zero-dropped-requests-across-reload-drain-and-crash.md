---
id: 10
uid: 4e0c7af5-c85e-4702-8ec9-37d881a10965
key: zero-drop
title: Zero dropped requests across reload, drain and crash
type: budget
status: planned
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

## Promise

Under steady load:

- A config reload fails no requests.
- SIGTERM drains every in-flight request before exit, within the grace period.
- A liamd crash refuses no new connections. Requests in flight on the crashed process are lost, and this budget does not claim otherwise.

It does not cover VM reboots or A/B updates. Keeping traffic up across a replaced instance is the load balancer's job.

## How it is measured

A load generator runs at a fixed rate while the harness triggers a reload, a SIGTERM, and a SIGKILL of liamd in turn. The harness counts failed requests and refused connections separately.

## Gate

Zero failed requests for reload and drain. Zero refused connections for crash.

## Measured

Not yet measured.
