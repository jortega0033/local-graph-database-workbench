# Local Graph Database Workbench

An offline graph database and browser workbench with bounded queries, durable imports, and inspectable execution plans.

Only a runnable scaffold exists so far (help, version and stub commands, checked in CI); the product behaviour is planned. The scope, architecture and acceptance criteria are in [the project plan](docs/PROJECT-PLAN.md). The plan includes a reproducible verification strategy for the future build.

- [Product requirements](docs/PROJECT-PLAN.md#product-requirements)
- [Technical design](docs/PROJECT-PLAN.md#technical-design)
- [Implementation backlog and epic](https://github.com/jortega0033/local-graph-database-workbench/issues)

Estimated implementation and verification effort is 17–27 engineering hours.

## Program goal

Build a complete graph database in Rust: a durable storage engine, a standard graph query language with a cost-based optimiser, analytics on billions of edges, and a replicated mode, with a workbench that explains every plan.

The program is planned ([#39](https://github.com/jortega0033/local-graph-database-workbench/issues/39)). Phase 1 is the current epic [#1](https://github.com/jortega0033/local-graph-database-workbench/issues/1) and the only phase with tickets.

**Score.** Share of the openCypher compatibility test scenarios that pass, with query times on a generated social-network benchmark.

| Phase | Content | Hours (rough) | Issue |
|---|---|---|---|
| 1 | The current epic and its tickets | 56 to 96 | [#1](https://github.com/jortega0033/local-graph-database-workbench/issues/1) |
| 2 | Storage engine | 150 to 260 | [#40](https://github.com/jortega0033/local-graph-database-workbench/issues/40) |
| 3 | Query language | 150 to 250 | [#41](https://github.com/jortega0033/local-graph-database-workbench/issues/41) |
| 4 | Optimiser and runtime | 150 to 260 | [#42](https://github.com/jortega0033/local-graph-database-workbench/issues/42) |
| 5 | Analytics | 90 to 150 | [#43](https://github.com/jortega0033/local-graph-database-workbench/issues/43) |
| 6 | Transactions and replication | 160 to 280 | [#44](https://github.com/jortega0033/local-graph-database-workbench/issues/44) |

About 760 to 1300 engineer hours in total: rough estimates, not measurements. The numbers in the score and the finish lines are targets until a run shows them. Only phase 1 has tickets to work; the sub-issues of a later phase are written when it starts. No single session is expected to reach the end.

## License

[MIT](LICENSE) © 2026 Jake Ortega.
