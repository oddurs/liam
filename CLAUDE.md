# Working in liam

liam is an OS image for serving websites: a Rust PID 1 and web server on a
stripped Linux kernel. Speed is held to budgets that CI gates enforce.

## Rules

1. **Never name an assistant.** No co-author trailers, "generated with" footers,
   robot emoji, or mentions of Claude, Anthropic or AI in commits, pull requests,
   comments or docs. The `commit-msg` hook rejects them.
2. **Never commit to `main`.** It advances only through a merged pull request.
3. **One unit of work, one worktree, one branch, one pull request.** Never share a
   checkout with another agent.
4. **Green before a pull request.** `scripts/task check` must pass. Never
   `--no-verify`, never `|| true`.

## The workflow

```sh
scripts/agent start <type>/<slug>    # type: feat fix chore docs perf refactor test
cd ../.worktrees/liam/<type>/<slug>  # the path it prints
scripts/agent check
scripts/agent commit "type(scope): subject"
scripts/agent pr
scripts/agent done <type>/<slug>     # after merge, from the primary checkout
```

When the work has a cairn item, the slug starts with its number
(`fix/0041-slowloris-timeout`) and the commit body ends with `Refs: 0041`.

## The seam

CI and the hooks know only these verbs. Change the toolchain here, nowhere else.

```sh
scripts/task fmt | fmt:check | lint | test | build | check
```

## Commits

Conventional Commits, imperative, subject 72 characters or fewer, no trailing
period. The body says why.

## Budgets

If a change touches boot, image size, listen time, connection handling or the
static hot path, measure before and after and put the numbers in the pull
request under "Budget impact".
