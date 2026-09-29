# Contributing

## Setup

```sh
git clone git@github.com:oddurs/liam.git
cd liam
scripts/setup          # wires core.hooksPath to .githooks; run once
scripts/agent doctor   # verifies git, gh, the toolchain and the hooks
```

## The workflow

One unit of work is one branch, in one worktree, with one pull request. `main`
advances only through a merged pull request: the `pre-push` hook and branch
protection on GitHub both refuse a direct push.

```sh
scripts/agent start fix/0041-slowloris-timeout   # prints a worktree path
cd ../.worktrees/liam/fix/0041-slowloris-timeout

# work

scripts/agent check                              # fmt, lint, test, build
scripts/agent commit "fix(http): close connections at the header timeout"
scripts/agent pr
```

After it merges, from the primary checkout:

```sh
scripts/agent done fix/0041-slowloris-timeout
```

Branches are `<type>/<slug>`, with type one of `feat fix chore docs perf refactor
test`. When the work has a cairn item, the slug starts with its number.

This is a solo-maintainer repository, so pull requests need no approving review.
They do need a green `required` check, and they are still the only way into
`main`.

## Commits

[Conventional Commits](https://www.conventionalcommits.org): imperative, subject
under 72 characters, no trailing period. The body explains why; the diff already
says what. Reference the cairn item in a trailer:

```
perf(pack): align entries to 4 KiB so sendfile needs no bounce buffer

Refs: 0019
```

The `commit-msg` hook enforces the format.

## Checks

Everything goes through one seam, so CI and your machine cannot disagree:

```sh
scripts/task check     # fmt:check, lint, test, build
```

Never use `--no-verify`. If a check is wrong, fix the check in its own pull
request. A bug fix arrives with the test that would have caught it.

## Budgets

A change that touches the boot path, the image size, or the request hot path
says so in the pull request, with a measurement before and after. Once the gates
exist, CI enforces them.

## The backlog

Issues and the roadmap are [cairn](https://github.com/oddurs/cairn) items under
`cairn/items/`, versioned with the code. `ROADMAP.md` is generated from them, so
never edit it by hand.

```sh
cairn next             # what is ready to start
cairn prompt 12        # one item, with everything it rests on
```
