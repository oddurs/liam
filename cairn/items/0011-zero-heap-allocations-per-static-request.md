---
id: 11
uid: 45334d50-5338-44f1-94a1-830d8e9338d8
key: no-alloc
title: Zero heap allocations per static request
type: budget
status: planned
created: 2026-09-28
updated: 2026-09-28
priority: p2
---

## Promise

After warm-up, serving a static file from the pack performs no heap allocation, over HTTP/1.1, HTTP/2 and TLS.

## How it is measured

A counting global allocator in an integration test. The test serves 10,000 requests across the fixture site after warm-up and asserts the count did not change.

## Gate

The count is exactly zero.

## Measured

Not yet measured.
