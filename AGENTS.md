# AGENTS.md

The full repository instructions are in [CLAUDE.md](CLAUDE.md). This file repeats only what an agent needs before it reads that.

## Status

Only the scaffold exists, with no product behaviour. The repository holds the plan (`docs/PROJECT-PLAN.md`), the delivery tooling, the backlog (epic #1) and a runnable Rust scaffold. Rust is not installed on the owner's PC; Rust checks run in CI only.

## Commands (the first four are real in the scaffold, verified in CI only; the rest are planned)

```
cargo build --frozen
cargo fmt --check
cargo clippy --frozen -- -D warnings
cargo test --frozen
./target/debug/waymark demo
./target/debug/waymark init <db>
./target/debug/waymark query <db> <query>
./target/debug/waymark serve <db>
```

Rust standard library only, no Cargo dependency, no package download, no JavaScript build step. The server binds `127.0.0.1` only.

## Ticket work

To implement a GitHub issue, follow the steps in [.claude/skills/implement-ticket/SKILL.md](.claude/skills/implement-ticket/SKILL.md) and the section "Agent roles" in [CLAUDE.md](CLAUDE.md). The role files in `.claude/agents/` each start with a section "In this repository" that holds the project rules for that role. An agent that cannot delegate to those roles does the before-coding, while-coding and specialist steps itself and says so in the pull request. The review gate and the acceptance gate still apply to every ticket, but the agent that wrote the change cannot pass them: do not push or open the pull request. Stop, report that the gates could not run, and leave the run for the owner. The order is: local evidence, both gates, draft pull request, CI, then ready only on green CI.

## Loop conventions

- The loop runs at L1: report-only, started by hand. It proposes; it does not label, comment, close, edit source, push or merge.
- No scheduler and no loop workflow is installed. Do not add one without the owner's approval.
- See [LOOP.md](LOOP.md) for the manual command and the human gates.
