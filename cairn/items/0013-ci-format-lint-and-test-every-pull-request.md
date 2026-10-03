---
id: 13
uid: 260e57f1-6e39-40c6-8479-549fb0dd9c25
title: 'CI: format, lint and test every pull request'
type: chore
status: done
milestone: v0.1
assignee: Oddur Sigurdsson
depends_on:
- 12
created: 2026-09-28
updated: 2026-10-02
closed_at: 2026-10-02
priority: p0
area: infra
effort: s
---

## Purpose

Pull requests become the only way `main` moves, so the checks have to run on them.

## Approach

A GitHub Actions workflow that runs `scripts/task check` on pull requests and on `main`, with cargo caching. CI knows only the task verbs, so it cannot drift from local runs.

## Acceptance criteria

- [x] The workflow runs `scripts/task check` and nothing else that duplicates it.
- [x] A pull request with a clippy warning fails CI.
- [x] A cached run finishes in under 5 minutes.

## 2026-10-02

Criterion 1: .github/workflows/ci.yml (from bootstrap) runs on pull_request and push to main. Job 'check' runs exactly one command, ./scripts/task check, after checkout, Swatinem/rust-cache and the pnpm/Node setup the site needs. The only other run steps are the 'static' job (apt musl-tools; scripts/task build:static, a release musl build that check does not do) and the 'required' aggregator. Nothing re-runs fmt, clippy, test or build outside the task verb.

## 2026-10-02

Criterion 2: canary PR #6 (branch test/0013-clippy-canary, made through the GitHub contents API so no local hook was bypassed) changed liam's version_line to 'return format!(...);'. Locally rustc gave 0 warnings and clippy failed on clippy::needless_return. CI run 37085900711 on 1feb7cb: check FAILURE (scripts/task check exit 101, needless_return under -D warnings), required FAILURE, PR mergeStateStatus BLOCKED. main's protection requires the 'required' check, strict. PR closed and branch deleted.

## 2026-10-02

Criterion 3: cached PR run 37085707747 (e937e42): rust-cache 'full match: true' on v0-rust-check-Linux-x64-516c6f62-9c483a5d, pnpm cache hit. check job 01:20:25-01:21:01 (36 s), whole run 01:20:22-01:21:08 (46 s). Run 37085478213: check job 36 s. The 5-minute bound has about 6x headroom at the current workspace size.

## Result

The ci workflow from bootstrap already met the item, so the work was proving it. On every pull request and push to main, its check job runs ./scripts/task check and nothing that duplicates it. The static job runs build:static, a separate verb. A canary PR (#6) added a needless_return, which rustc accepts and clippy rejects. CI failed it in check, and main's protection, which requires the 'required' check, blocked the merge. Cached PR runs restore the cargo and pnpm caches on a full key match: the check job takes 36 s and the whole run 46 s, well under the 5-minute bound. No workflow change was needed.
