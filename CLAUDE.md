# CLAUDE.md

Working instructions for this repository. Read this file first, then the documents it links to. It does not repeat them.

## What this is

An offline workbench for a developer with a small graph: load sample relationships, run a bounded one- or two-hop query in the browser or the command line, see the exact edges and the access path, import whole batches durably, restart and get the same answer. Rust standard library only, `127.0.0.1` only, Linux or WSL2.

**Nothing is implemented yet.** The repository holds the plan, the delivery tooling and the backlog in epic #1. Commands marked planned below do not work yet. Do not describe a planned thing as working, and do not claim a passing baseline: there is none.

## Sources of truth

| Topic | Document |
|---|---|
| Product scope, design, fixtures with expected values | [docs/PROJECT-PLAN.md](docs/PROJECT-PLAN.md) |
| Planned module map, freeze points, planned versus implemented | [docs/architecture.md](docs/architecture.md) |
| Backlog | Epic issue #1 and its child tickets on GitHub. The epic's table is authoritative for order, dependencies and estimates; tickets state their own owner-decision defaults. |
| Loop setup and its limits | [LOOP.md](LOOP.md), [loop-constraints.md](loop-constraints.md) |
| Project subagents, origin and local changes | [.claude/agency-agents/README.md](.claude/agency-agents/README.md) |
| Ticket workflow and the step at which each role runs | [.claude/skills/implement-ticket/SKILL.md](.claude/skills/implement-ticket/SKILL.md), section "Agent roles" below |
| How a ticket goes from the queue to a ready pull request, who may do what | "Delivery workflow" below |
| UI and copy rules, design tokens | [docs/ui-quality.md](docs/ui-quality.md), [docs/design-tokens.md](docs/design-tokens.md) |
| Decisions the owner made, with dates | [docs/owner-decisions.md](docs/owner-decisions.md) |

The plan wins for product design; the epic and tickets win for order and scope. A ticket that changes a contract fixes the plan in the same pull request.

## Stack and constraints

- Rust 1.89 or newer, edition 2021, **no Cargo dependencies**, committed lockfile, no toolchain override file.
- One package with a library and one binary that embeds the HTML, CSS and JavaScript. Native SVG, DOM and fetch; no JavaScript build step.
- Supported platform: Ubuntu in WSL2 or native Linux, database on its Linux filesystem. Loopback only, HTTP is read-only.
- Offline: no registry, no CDN, no service account.
- Editing or deleting in place, openCypher compatibility, remote servers, authentication, concurrent writers, replication and compaction are outside this release.
- No latency claim; counters are exact and deterministic.
- A runtime version change, a dependency, an external request or a non-loopback listener needs the owner's approval first, not a pull request.

## Commands

Rust is not installed on the owner's PC and nothing is installed there. **Rust checks run in CI only.** Run `cargo --version` first: if it is missing, do not install it, and do not claim you ran any command below.

| Purpose | Command | Status |
|---|---|---|
| Build | `cargo build --frozen` | planned: real with the scaffold |
| Format check | `cargo fmt --check` | planned: real with the scaffold |
| Lints | `cargo clippy --frozen -- -D warnings` | planned: real with the scaffold |
| Tests | `cargo test --frozen` | planned: real with the scaffold |
| Demo with the bundled seed | `./target/debug/waymark demo` | planned: #2 |
| Create and load a database | `./target/debug/waymark init <db>` | planned: #4 |
| Query | `./target/debug/waymark query <db> <query>` | planned: #5 |
| Serve a database | `./target/debug/waymark serve <db>` | planned: #4 and #6 |

## Architecture boundaries

The planned module map is in [docs/architecture.md](docs/architecture.md). Invariants every change must keep (the plan is the source):

- Parsing, graph, execution, index, journal, HTTP and command line stay separate. Parsing and traversal know nothing of HTTP, import or storage.
- Standard library only, edition 2021, a lockfile that lists only this package, and no toolchain file that could trigger a download.
- The grammar is exactly the plan's subset. Anything else is a stable error code with a zero-based byte offset. Keywords are uppercase, aliases are `a`, `b`, `c`, LIMIT is 1 to 100 with default 20.
- Rows are sorted by the numeric tuple `[a, first_edge, b, second_edge?, c?]` before truncation. Parallel edges are distinct rows; a walk may reuse a node or edge.
- Types never coerce: text `"1"` and integer `1` differ, and a missing key or other type is false.
- An executed query inspects at most 100,000 start and adjacency candidates; past that it returns `QUERY_LIMIT` and no rows. `EXPLAIN` traverses no edge.
- A batch is validated as a whole before one complete frame is appended under the exclusive lock; success is reported only after `File::sync_all()`. A write or sync error is an indeterminate outcome and closes the file.
- Only an incomplete final frame is a recoverable tail. A complete frame with a bad length, checksum, payload or invariant stops replay with its byte offset; nothing is repaired silently. `123456789` has CRC `0xCBF43926`.
- The index is derived from the accepted graph at load or replay and has no disk state. Index and forced scan give the same rows.
- Bind `127.0.0.1` only; header block 8 KiB, query 4 KiB, one Content-Length, no Transfer-Encoding, finite deadlines, one response per connection, no CORS. HTTP is read-only.
- Import caps: 1 MiB and 1,000 records per batch, 10,000 nodes, 50,000 edges, 64 MiB journal; text up to 256 bytes without control characters.
- Query values, names and errors go into the page as DOM text nodes, never as HTML. An error message never prints payload data.
- Every visual path has a textual row in a keyboard-accessible table; errors and results use a status region; clipping (100 nodes, 150 edges) is reported; a zero-row result clears the old one.
- The reference enumerator in tests uses direct joins over raw rows and shares no parser, adjacency or sort code with the engine.

## Git workflow

- `main` is the default branch. Work on one branch per ticket: `<issue-number>-<short-slug>`. Never commit to `main`.
- Small commits with imperative subjects and a type prefix (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).
- Order for every ticket: local evidence, Code Reviewer, Reality Checker, then a draft pull request that says `Part of epic #1` and `Closes #<ticket>`, then CI on that draft, then `gh pr ready` only when CI is green.
- Do not push or open a pull request unless the owner asked in the current session; starting `/next-ticket` is that request for one ticket. Only the owner merges and releases. Do not push to `main`, force-push, or merge your own pull request.
- Never commit a database file (`data/`, `*.wmk`), `.env*`, or build output.
- Do not change `.claude/agents/`, `LOOP.md`, `loop-constraints.md` or `loop-budget.md` as part of a feature ticket.
- Open owner decisions on a ticket are settled with the ticket's stated default. Where a ticket states none, take the conservative option and say so in the pull request.

## Evidence before a ticket is called done

Paste real output into the pull request; never summarise a run you did not make.

1. For each acceptance criterion of the issue: the name of the test that proves it, or the exact step and what was observed.
2. Rust results come from CI, and green CI on the current diff is a condition for `gh pr ready`: the link to the CI run on the draft pull request with the Rust version and the results of `cargo fmt --check`, `cargo clippy`, `cargo build` and `cargo test --frozen`. If Rust is not available locally, say so.
3. For a query change: the plan's fixture query and the exact tuples, and the comparison with the independent enumerator.
4. For a journal change: the fault (byte position, corruption, injected error), the recovered state and the fault model.
5. For a UI change: the browser and version, the steps and what was observed, and the manual steps left as owner checks.
6. `git status` shows no stray files, and no dependency, external URL or non-loopback listener was added.
7. Anything not verified, said plainly.

## Agent roles

Project subagents are in `.claude/agents/`: nine Agency Agents roles and `loop-verifier`, which belongs to the Loop setup. Each Agency Agents role file starts with a section "In this repository", written for this project. That section and this file override the generic text below it. Sources, licence, local changes and the name collision with a user-level install are in [.claude/agency-agents/README.md](.claude/agency-agents/README.md).

When asked to implement, continue or finish a GitHub issue, follow the playbook [`.claude/skills/implement-ticket/SKILL.md`](.claude/skills/implement-ticket/SKILL.md). Claude Code loads it on its own for ticket work; `/implement-ticket <issue number>` starts it by hand. Delegate to the roles without waiting to be asked: each role's `description` ends with a "Use proactively ..." sentence, which is what makes Claude Code delegate. The playbook gives the exact trigger of each role.

| When | Roles | Access |
|---|---|---|
| Before coding | Software Architect | `Read, Grep, Glob` |
| While coding | Backend Architect, Database Reliability Engineer, Frontend Developer | can edit |
| After coding, when the trigger matches | Application Security Engineer, Accessibility Auditor | `Read, Grep, Glob` |
| After coding, when the trigger matches | API Tester | can edit |
| Gate on every ticket | Code Reviewer on the full diff, until no material finding is left | `Read, Grep, Glob` |
| Acceptance gate on every ticket, after Code Reviewer | Reality Checker on each acceptance criterion and the recorded evidence, and that CI is configured to run the checks (no CI result is needed yet) | `Read, Grep, Glob` |

- The two gates run on every ticket, whatever its size. A ticket is not called done before both have passed.
- Roles limited to `Read, Grep, Glob` cannot run commands. Give them the diff, the acceptance criteria and whatever command output exists.
- The session or role that wrote a change does not count as its reviewer.
- If a role is missing or cannot be delegated to, the session does the before-coding, while-coding and specialist steps itself and says so. It cannot pass the two gates on its own change: do not push or open the pull request. Stop, report that the gates could not run, and leave the run for the owner. This overrides the playbook's sentence "do that step yourself" for the two gates.
- In the pull request description, list under "Agent roles run" each role that ran, what it was asked and its verdict, and any step done without a role.

## Delivery workflow

How a ticket reaches the owner. Each piece is described in its own file; this part only links them.

- **Start:** the owner runs `/next-ticket` ([`.claude/skills/next-ticket/SKILL.md`](.claude/skills/next-ticket/SKILL.md)); it picks one leaf ticket (never a `tracking` parent or a ticket with open sub-issues; the last sub-issue's pull request also closes the parent), creates the branch and worktree, and runs the playbook [`.claude/skills/implement-ticket/SKILL.md`](.claude/skills/implement-ticket/SKILL.md). Nothing is scheduled.
- **Gate hook:** [`.claude/hooks/role_gate.py`](.claude/hooks/role_gate.py) blocks `gh pr create` and `gh pr ready` until Code Reviewer and Reality Checker ran after the last code edit. `ROLE_GATE=off` in the environment Claude Code is started from switches it off for that session; only the owner does that.
- **Permissions:** [`.claude/settings.json`](.claude/settings.json). Agent configuration edits ask in the main checkout and in ticket worktrees; CI, `CLAUDE.md`, `AGENTS.md` and Loop-file edits ask only in the main checkout; in a ticket worktree they are limited by the playbook's scope rule and reviewed by the owner at merge (shell-made edits are not covered, so a ticket never changes agent configuration); force pushes, pushes to `main`, `gh pr merge`, releases and reads of `.env` are denied.
- **CI:** [`.github/workflows/ci.yml`](.github/workflows/ci.yml). Every job must be green on the current diff before a pull request is marked ready. The agent does not poll: it ends its turn with "Draft pull request #N is waiting for CI" and reads `gh pr checks` once when woken. Reviewers read the diff and evidence from `.claude/state/review/`. An acceptance step that needs a person is listed under "Owner checks" in the pull request.
- **Pull request:** [`.github/pull_request_template.md`](.github/pull_request_template.md). Tickets use [`.github/ISSUE_TEMPLATE/implementation-ticket.md`](.github/ISSUE_TEMPLATE/implementation-ticket.md).
- **Labels:** `status:ready`, `status:blocked`, `status:in-progress`, `needs-decision`, `tracking` (parent container, never worked; its sub-issues are the work items).
- **Changelog:** every ticket pull request adds one line under Unreleased in [CHANGELOG.md](CHANGELOG.md).
- **UI and copy:** [docs/ui-quality.md](docs/ui-quality.md); the `ui-ux-pro-max` and `unslop` skills are in `.claude/skills/`. Colours come from the shared design tokens in `web/tokens.css` ([docs/design-tokens.md](docs/design-tokens.md)); new UI adds no colour literals.
- **Decisions:** [docs/owner-decisions.md](docs/owner-decisions.md). Security reports: [SECURITY.md](SECURITY.md).
- **Merge and release:** only the owner merges and releases ([docs/RELEASING.md](docs/RELEASING.md)). An agent stops at a ready pull request.

## Loop

Issue triage runs at L1: report-only, started by hand, no scheduler, no GitHub writes. See [LOOP.md](LOOP.md).
