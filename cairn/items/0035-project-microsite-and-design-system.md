---
id: 35
uid: 7e768140-8252-4f28-bdea-5cdeebceffaa
title: Project microsite and design system
type: feature
status: doing
milestone: v0.1
created: 2026-09-28
updated: 2026-09-28
priority: p2
area: site
effort: m
---

## Problem

People who care about fast infrastructure need to understand liam in one page, and the project needs a visual identity it can reuse.

## Proposal

An Astro static site in `site/` that ships zero framework JavaScript. It is built from a small design system of tokens, layout primitives and components, with a `/design` reference page. Content follows the published landing page. A static build is also what liam itself will serve later.

## Budget impact

None. The site is a static build, so liam can serve it.

## Acceptance criteria

- [ ] `pnpm check` and `pnpm build` pass in `site/`.
- [ ] The landing page renders the same sections as the published artifact.
- [ ] `/design` shows every token and component.
- [ ] Pages ship no JavaScript except the boot-trace replay.

## 2026-09-28

Built on branch feat/site (worktree ../.worktrees/liam/feat/site): Astro 7, zero framework JS, pnpm check and build pass. Rough: roadmap section hard-coded, some raw px values outside tokens.
