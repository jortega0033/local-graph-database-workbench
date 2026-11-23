# Owner decisions

A dated record of the decisions made for this repository, so the rules do not depend on anyone's memory of a conversation. Changing any entry needs the owner. An agent that finds a decision blocking a ticket stops and says so.

The owner made decisions 1 to 10 and 12 on 2026-11-22 for all repositories that get the SDLC kit, and asked on 2026-11-23 to apply the kit here (jortega0033/sdlc-kit v1.1.0). Each row says who decided.

| # | Decision | Source | Date | Implemented in |
|---|---|---|---|---|
| 1 | The instructions, tooling and playbook live on the default branch (`main`). | decided by the owner | 2026-11-22 | `CLAUDE.md`, `.claude/`, `.github/` |
| 2 | Persona agents run automatically during ticket work and carry instructions tailored to this project. | decided by the owner | 2026-11-22 | `.claude/agents/`, `.claude/agency-agents/README.md`, `CLAUDE.md` "Agent roles" |
| 3 | Every open ticket is about app implementation; tooling is installed, not ticketed. | decided by the owner | 2026-11-22 | `.claude/`, `.github/`, `docs/` |
| 4 | Agents may take a ticket up to a ready pull request. Only the owner merges and releases. | decided by the owner | 2026-11-22 | `CLAUDE.md`, `.claude/skills/next-ticket/SKILL.md`, `docs/RELEASING.md`, `.claude/settings.json` |
| 5 | No scheduler. Ticket work starts only on the owner's go signal (`/next-ticket`), because the repository is used to train AI models. | decided by the owner | 2026-11-22 | `.claude/skills/next-ticket/SKILL.md`, `LOOP.md` |
| 6 | A review gate hook blocks opening or readying a pull request until the gate roles have run after the last code edit. | decided by the owner | 2026-11-22 | `.claude/hooks/role_gate.py`, `.claude/settings.json` |
| 7 | Loop Engineering stays at L1, report-only and unscheduled. | decided by the owner | 2026-11-22 | `LOOP.md`, `loop-constraints.md`, `loop-budget.md` |
| 8 | Continuous integration runs on Linux and Windows; the default branch requires a pull request and green checks. The product targets Linux: Rust checks run on Linux, and the gate hook also runs on Windows. | decided by the owner | 2026-11-22 | `.github/workflows/ci.yml`; branch protection is a repository setting |
| 9 | Permission rules: documented commands are allowed; force pushes, pushes to the default branch, merges, releases and reads of secrets are denied; edits to CI, agent configuration and Loop files ask first. | decided by the owner | 2026-11-22 | `.claude/settings.json` |
| 10 | Open decisions on tickets are settled with the ticket's stated default; where a ticket states none, the conservative option is taken and flagged in the issue. | decided by the owner | 2026-11-22 | The list below; the issues themselves |
| 11 | Rust is verified in CI only; nothing is installed on the owner's PC. The crate has no dependencies. | decided by the owner (2026-11-22, entry 11 of the kit); repeated here for this repository | 2026-11-23 | `CLAUDE.md` "Commands", `.github/workflows/ci.yml` |
| 12 | The ten UI rules in `docs/ui-quality.md` are binding, and the `ui-ux-pro-max` and `unslop` skills are installed in the repository and used for UI work and for copy. | decided by the owner | 2026-11-22 | `docs/ui-quality.md`, `.claude/skills/` |
| 13 | Visual system: the Pipenzo Flow look tokens. | decided by the owner | 2026-11-23 | `web/tokens.css`, `docs/design-tokens.md` |
| 14 | Token rules 14-D1 to 14-D4. | 14-D1 use of yellow decided by the owner; how it is drawn, and 14-D2 to 14-D4, recommended by the orchestrator and adopted under the owner's delegation of 2026-11-23 | 2026-11-23 | `docs/design-tokens.md`; applied in UI tickets |

## Open ticket decisions settled by default (entry 10)

The plan leaves these open. Each is a recommendation by the orchestrator, adopted under entry 10 on 2026-11-23, and holds unless the owner changes it in the issue. None blocks a ticket.

| Issue | Decision | Default |
|---|---|---|
| #2 | Rust version | `rust-version = "1.89"` in `Cargo.toml`. CI installs 1.89.0 with `rustup`. No `rust-toolchain.toml`, because the plan forbids an override that downloads. |
| #2 | Package and binary name | Package and binary `waymark` (the plan's product name is Waymark Graph). |
| #2 | Port | `serve` and `demo` take `--port`, default 0 (a free port); the URL is printed. |
| #3 | Error codes | Upper-case words (`QUERY_LIMIT`, `QUERY_SYNTAX`, `QUERY_TOO_LARGE`, `LIMIT_RANGE`); #2 writes the full list in `docs/architecture.md`, #3 extends it. |
| #4 | Journal header | Eight ASCII bytes `WAYMARK1` followed by a little-endian 32-bit format version 1. |
| #4 | Lock | `File::try_lock` and `File::try_lock_shared` on the journal file (stable since Rust 1.89). A busy lock is an error, never a wait. |
| #5 | Plan choice on a tie | Equal cardinality uses the label scan, as the plan says ("smaller than"). |
| all | Owner checks | Manual browser steps run in WSL2 or on a Linux host. |
| none | Windows CI | Extends entry 8: the `rust` job also runs on `windows-latest`, only as a build and test of the portable scaffold. Linux is the supported platform. |
| none | Licence | MIT, in `LICENSE` on `main` since before the kit. No ticket changes it. |
