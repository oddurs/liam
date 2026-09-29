---
id: 51
uid: 64ea6b4a-0844-4c4f-a300-7150fcf191eb
title: What zero dropped connections means
type: decision
status: planned
milestone: v0.3
created: 2026-09-28
updated: 2026-09-28
priority: p0
area: docs
effort: s
budget:
- zero-drop
---

## Context

The landing page promises "0 dropped connections on config reload, binary upgrade, or a liamd crash". Two parts of that cannot be true as written:

- A crashed process loses the requests it was serving.
- An immutable image has no in-place binary upgrade. A new image means a new VM or container.

## Options and tradeoffs

1. Keep the claim and add in-place upgrades by pushing binaries to a running instance. This breaks immutability, which is the stability story.
2. Narrow the claim to what can be proven: no failed requests across reload and SIGTERM drain, and no refused connections across a liamd crash. Leave instance replacement to the load balancer.

## Decision

Option 2. The `zero-drop` budget states it that way, and the site copy changes to match.

## Revisit when

liam gains a supported way to change a running instance's pack, such as a pack swap in the reload spike.

## Acceptance criteria

- [ ] The budget item and the site copy match this decision.
