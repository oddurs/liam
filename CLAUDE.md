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

<!-- cairn:begin -->
## Roadmap and issues

This project tracks its roadmap and issues with `cairn`. Every item is a Markdown file under `cairn/items`, described by the schema in `cairn.toml`.

**Do not create ad-hoc TODO, PLAN or NOTES files.** Create a cairn item instead, so the work appears on the board and in the generated roadmap.

### The loop

1. `cairn next` — what is ready to start. It excludes anything blocked by unfinished dependencies and puts work already in progress first.
2. `cairn claim <ID>` — take it before you start, so no one duplicates the work. `cairn claim --next` picks and claims the top-ranked unclaimed item in one step. Then read `cairn prompt <ID>`: the item with everything it rests on — the outcome it serves, what its dependencies concluded, what done means and what earlier runs learned.
3. Do the work. Record what you learn: `cairn set <ID> <field>=<value>` for fields, `cairn note <ID> "<TEXT>"` for anything that needs a sentence — why you chose something, what you tried, what to watch for.
4. `cairn tick <ID> <N>` as each acceptance criterion becomes true — `cairn show <ID> --criteria` lists them numbered. Tick what is true, not what would let you close.
5. `cairn close <ID>` when it is done, or `cairn release <ID>` to hand it back.
6. `cairn check` before you report finished. It must pass.

### Commands

```sh
cairn next --json                 # ready work, ranked
cairn claim --next                # take the next ready item
cairn search <TEXT> --json        # titles, bodies and labels
cairn list --json                 # all open items
cairn list --filter 'blocked=false,priority=p0'
cairn prompt <ID>                 # the item as a prompt, with what it rests on
cairn show <ID> --json            # one item, including its body
cairn new "<TITLE>" --type <TYPE> --milestone <MILESTONE>
cairn set <ID> status=<STATUS>    # also labels+=x, or any field below
cairn note <ID> "<TEXT>"          # append reasoning; never replaces
cairn show <ID> --criteria        # acceptance criteria, numbered
cairn tick <ID> <N>               # tick one; --all for every one
cairn close <ID>
cairn check                       # validate; run before finishing
cairn render                      # regenerate ROADMAP.md
```

Claims coordinate writers in the same item directory and are seen across the worktrees of this repository: `next` leaves out what another worktree has claimed, `claim` refuses it, and `cairn worktrees` shows what each is doing. Separate clones are not read. Agree on assignments before splitting work across clones.

Items are numbered: `0012`, and commands accept the bare number too. Write the number in `depends_on` and other id references. Each item also carries a `uid` tag; leave it alone. If a merge gives two items one number, `cairn renumber` moves the one that arrived and retargets the references that came with it.

### Schema

- **Types**: `feature`, `bug`, `spike`, `chore`, `docs`, `decision`, `milestone`, `budget`
- **Statuses**: `backlog` (open), `planned` (open), `doing` (active), `blocked` (open), `done` (done), `dropped` (dropped)
- **`priority`**: one of p0, p1, p2, p3 — Relative to the item's milestone. p0: the milestone does not ship without it; p1: expected in it; p2: useful; p3: optional
- **`effort`**: one of s, m, l, xl — Rough size, never an estimate. s: hours; m: a day; l: several days; xl: split it or spike it
- **`area`**: one of kernel, init, io, http, tls, router, pack, config, proxy, image, cli, deploy, bench, ops, test, security, infra, site, docs — Subsystem this touches
- **`due`**: date, YYYY-MM-DD — A committed external date, YYYY-MM-DD. Leave unset when the release is gated by evidence
- **`part_of`**: names any items, by id, several allowed — A larger piece of work this belongs to
- **Milestones**: `v0.1`, `later`, `v0.2`, `v0.3`, `v0.4`, `v1.0`
- **Saved views** (`cairn list --view NAME`): `now`, `next`, `release`, `spikes`, `waiting`, `dependencies`, `triage`, `later`, `decisions`, `history`

### Rules

1. Before starting work, find or create the item and set it to an active status.
2. Use the fields above rather than inventing new ones; add new fields to `cairn.toml` first.
3. Never hand-edit the generated roadmap file — change items and run `cairn render`.
4. `cairn check` must pass before the work is considered done.

<!-- cairn:end -->
