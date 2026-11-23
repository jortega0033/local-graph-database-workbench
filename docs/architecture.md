# Implementer map

Status: **planned, not implemented.** This page maps the boundaries that [PROJECT-PLAN.md](PROJECT-PLAN.md) requires onto a module layout so that parallel tickets do not collide. It adds no requirement; the plan wins where they differ. The names are the orchestrator's proposal, fixed by the scaffold; a ticket that changes one corrects this page in the same pull request.

## Planned module map

| Boundary | Module | Owns | Parent ticket |
|---|---|---|---|
| Entry point and CLI | `src/main.rs, src/cli.rs` | Subcommands `demo`, `init`, `load`, `query`, `serve` | #2 (`demo`), #4 (`init`, `load`), #5 (`query`, `serve` diagnostics) |
| Graph | `src/graph.rs` | Typed nodes, edges, scalars, ID namespaces, adjacency | #2 |
| Response contract | `src/response.rs` | Rows, projected aliases, plan, counters, error codes | #2 |
| Query language | `src/query.rs` | Tokenizer, parser, query model, byte offsets | #3 |
| Execution | `src/exec.rs` | Scan, traversal, ordering, budgets, counters | #3, #5 |
| Batch import | `src/batch.rs` | TSV parsing and whole-batch validation | #4 |
| Journal | `src/journal.rs` | Header, frames, CRC, locks, replay, tail recovery | #4 |
| Index | `src/index.rs` | Typed equality index, plan choice | #5 |
| HTTP | `src/http.rs` | Loopback server, limits, embedded assets | #2 |
| Browser assets | `web/ (embedded)` | Query editor, table, path preview, diagnostics | #2, #6 |
| Test oracle | `tests/oracle/` | Independent path enumeration | #3 |

Dependency direction: `cli` and `http` depend on `exec`, `journal` and `response`; `exec` on `query`, `graph` and `index`; `journal` on `batch`; `batch` and `index` on `graph`. `graph` and `query` import nothing else from the crate. The crate has a library and a thin binary so integration tests can use it.

## Freeze points

Parallel tickets rely on these shapes. A ticket that changes a frozen shape updates this page and every dependent ticket in the same pull request.

- **Graph types and IDs**, and the **response contract** (success and error fields): ticket #2, before #3 and #4 start.
- **Query model** (parsed form): ticket #3.
- **Journal format version 1**: ticket #4.
- **Diagnostics fields** (access path, counts): ticket #5; #6 may build on fixed samples of the same shape.

## What is verified where

Rust is verified in CI only (see [CLAUDE.md](../CLAUDE.md)), on Linux, the supported platform. Interruption tests are simulated (truncated and corrupted files, injected errors); they are not a power-loss guarantee. Manual browser steps are owner checks in WSL or on Linux.
