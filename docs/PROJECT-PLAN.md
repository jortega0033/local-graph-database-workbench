# Local Graph Database Workbench

> This document specifies phase 1 of the program in [#39](https://github.com/jortega0033/local-graph-database-workbench/issues/39). Hour figures in it are early estimates for that phase; its open tickets add up to 56 to 96 hours (2026-11-24). Work that it defers is planned in the later phases only where the program names it.

Product and engineering plan for Waymark Graph. The browser workbench, storage layer and query engine are planned.

## Product requirements

A developer examining a small dataset can load sample relationships, see exactly which edges connect two nodes, and repeat the same query after restarting. A second goal is to make a plan choice understandable by showing how many candidate start nodes this query actually inspected. The first release runs entirely on one machine with an installed Rust toolchain and a browser.

- As a developer exploring relationships, I can open a small sample, run a saved example and see the matching nodes and their distinct edges in a table and a graph.
- As a developer testing query behavior, I can change a label, filter, direction or limit and get a deterministic answer or an error pointing to the problem.
- As a developer importing a dataset, I can see whether a whole batch was committed. On restart, I can recover every acknowledged batch and check the same query again.
- As a developer learning how the engine works, I can see the access path and execution counters and compare an indexed run with a scan over the same data.

### First release and exit gate

The first usable prototype serves the graph and one read-only browser query over a bundled seed from localhost. The finished epic adds one- and two-hop traversal, typed filters, durable batch import, a derived index, plan diagnostics and offline verification. Editing nodes or edges in place, deletion, openCypher compatibility, remote servers, authentication, concurrent writers, replication, compaction and production database administration stay outside this release.

From a fresh checkout with a supported compiler installed, a user can build without package downloads, create a fresh database with a bundled batch, get the expected rows in the browser, add another batch through the CLI, restart, rerun the query, and compare scan and indexed results on the selective fixture. The offline tests and the manual walkthrough both pass. A failure message identifies which input or stored frame caused the problem without printing arbitrary payload data.

## Technical design

### Environment and interfaces

Target Ubuntu in WSL2 or native Linux, using a database path on its Linux filesystem. Use Rust 1.89+ and the standard library, one Cargo package with edition 2021 and a committed lockfile. Serve local HTML, CSS and browser JavaScript from the binary. Browser layout and transport use native SVG, DOM APIs and fetch. There are no Cargo dependencies, JS build steps, CDNs or service accounts. Run the already-installed toolchain offline. Provisioning a missing compiler is a separate prerequisite. Check `rustc --version`, `cargo --version` and browser availability before starting. Do not add a toolchain override that triggers automatic downloads. [Cargo offline and frozen options](https://doc.rust-lang.org/cargo/commands/cargo-test.html#manifest-options) · [Rust file locking](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock)

The target CLI has `demo` for the stock in-memory seed, `init <db>` to create a new journal, `load <db> <batch.tsv>` for a transaction, `query <db> <query>` for a single read, and `serve <db>` for a browser snapshot. The serve command prints its loopback URL. It loads under a short shared lock, then serves an immutable snapshot until the process restarts. Import is a local command, so the HTTP surface has only reads. Contract for app and CLI query responses includes ordered rows, projected aliases, optional plan and its counters, or a stable error code with an input position when available.

Bind to 127.0.0.1 only. Serve named local UI assets and `POST /api/query` with a UTF-8 text body. Support HTTP/1.1 requests with bounded headers and body, require a single Content-Length for POST, reject Transfer-Encoding and unsupported routes or methods, use finite read and write deadlines, and close each connection after one response. Cap the header block at 8 KiB and query text at 4 KiB. Browser code inserts query values and error text via DOM text nodes. Never embed them as HTML. Use a same-origin client with no CORS.

### Graph data and import contract

A graph contains immutable node IDs and separately unique immutable edge IDs, each a positive safe JavaScript integer. The same number can exist in both ID namespaces. A node has one ASCII label. An edge has a source, target and ASCII type. Parallel edges and self-loops are valid. Node properties are unique named scalars of type integer, boolean or UTF-8 text. Integers are signed in the safe JavaScript range. IDs within one kind cannot repeat across batches. Labels, edge types and keys start with one ASCII letter followed by up to 31 ASCII letters, digits or underscores. Labels, keys and text equality are case-sensitive.

The version-one batch format is line-oriented UTF-8 separated by tabs, with LF or CRLF line endings and optional final newline. A node line has `N <id> <label>` followed by zero or more `<key> <type> <value>` field triples. An edge line has `E <id> <from> <to> <type>`. Spaces in these examples mark field boundaries for readability. Only tabs delimit actual fields. Scalar tags are `int`, `bool` and `text`. Booleans are `true` and `false`. Text can be empty or up to 256 UTF-8 bytes and cannot contain control characters. Allow an entirely blank final line and reject any blank interior row, extra fields, duplicate keys or dangling endpoints. Endpoints may occur in any order within the same batch. Limit a batch to 1 MiB and 1,000 records, a graph to 10,000 nodes and 50,000 edges, and a journal to 64 MiB. All validation completes before an append.

### Bounded traversal language

These are the supported query shapes. This grammar describes a new subset for the workbench.

```text
[EXPLAIN] MATCH (a:Label)-[:TYPE]->(b:Label)
  [-[:TYPE]->(c:Label)]
  [WHERE a.key = literal] RETURN a,b [LIMIT n]

Two-hop queries use RETURN a,b,c.
Each hop can instead use <-[:TYPE]- for the backward direction.
literal = decimal integer | true | false | quoted UTF-8 text
```

The first, second and optional third aliases are exactly `a`, `b`, `c`. Keywords are uppercase, punctuation and token boundaries are explicit, and insignificant ASCII whitespace is allowed between tokens. A quoted literal allows escaped quote and backslash. Reject unknown escapes, trailing tokens, predicates other than equality on `a`, and anything beyond two hops. Error offsets are zero-based byte offsets into the UTF-8 input. Omitted LIMIT is 20, and explicit values are 1 through 100. Explicitly reject any syntax for writing data or evaluating arbitrary expressions.

An exact label and an appropriately directed edge type are required at every hop. Missing properties and mismatched value types evaluate false. Text `"1"` and integer `1` do not match. Rows carry the ordered node and edge IDs for a concrete path. Multiple edges between the same nodes remain distinct rows, and a two-hop walk may revisit a node or edge. Sort by the numeric tuple `[a, first_edge, b, second_edge?, c?]` before truncating. Cap combined start and adjacency candidate inspections at 100,000 for an executed query. On exhaustion return `QUERY_LIMIT` with no partial rows. For scan/index equivalence tests use bounded fixtures within both plans' budgets.

### Storage and recovery

Use an append-only transaction journal, which is also the replay source for the graph. One process can hold an exclusive write lock at a time using the journal file's OS lock. Readers fail cleanly on a busy lock rather than starting an inconsistent replay. External programs must not replace or modify the journal under a running process. Initialize with exclusive creation, write and sync the fixed version header, then sync the parent directory before reporting success on Linux. Every subsequent frame contains a leading body length and bitwise complement as little-endian 32-bit values, followed by the nonempty bounded batch bytes and standard CRC-32/ISO-HDLC of that body. Derive transaction order from frame sequence. There is no compaction.

Under the exclusive lock, replay known frames, validate the proposed batch including identity, endpoint and size constraints, and append exactly one complete frame. Acknowledge success only after `File::sync_all()` succeeds. A write or sync error returns an indeterminate outcome and closes the file. The CLI tells the caller to reopen the database and check known IDs before retrying. A complete valid frame can be present after an interrupted response. Repeating the same loaded batch is rejected as duplicate IDs.

Recover complete valid frames in order. An incomplete final length header, or a final body with a validated length and missing trailing bytes, is treated as a recoverable tail. A read reports the tail and serves the prior committed graph. The next writer truncates it to the last known valid boundary under an exclusive lock and synchronizes before appending. Complete frames with invalid length, checksum, payload or graph invariants stop replay with the byte offset. An invalid or truncated file header is a failure that requires operator intervention. Document the difference between simulated interruption/recovery checks and a physical power-loss guarantee. [Rust file sync](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) · [Linux directory sync semantics](https://man7.org/linux/man-pages/man2/fsync.2.html) · [CRC-32 reference algorithm and parameters](https://www.rfc-editor.org/rfc/rfc1952.html#section-8)

### Index, execution and browser

Build an in-memory adjacency list by direction and an equality index keyed by `(node label, property key, tagged value)` from the accepted graph during load or replay. Keep all IDs numeric and deterministic. When a predicate exists and its indexed cardinality is smaller than all nodes under the start label, choose the index. Otherwise choose the label scan. Retain a forced scan mode for local verification. The index is derived from the graph, so it has no separate disk state that can get out of step.

`EXPLAIN` returns the access path, start-label count, candidate count estimate, hop count, sort rule and limit without traversing any edges. A normal executed query returns rows plus actual counts of start candidates examined and adjacency candidates visited. The count excludes index build and the workbench makes no latency claim. The UI renders a fixed-layout seed view and a deterministic ID grid for larger data. Use a capped preview of at most 100 matched nodes and 150 matched edges and report when the preview is clipped. Keep the complete returned row list visible in a keyboard-accessible table with textual IDs and paths. Newly imported data appears after restarting the browser server.

A driver experiment belongs in a later epic. Check the intended driver and protocol version against Neo4j's published handshake and messaging specs, cover the required authentication and result states, and prove the exact driver install can run offline before claiming even one compatible combination. The browser and CLI have no such dependency. [Neo4j Bolt handshake](https://neo4j.com/docs/bolt/current/bolt/handshake/) · [Neo4j driver documentation](https://neo4j.com/docs/)

## Original fixtures and expected results

| Node ID | Label | Properties |
| --- | --- | --- |
| 1 | Person | `name="Ren"`, `group="blue"`, `rank=1` |
| 2 | Person | `name="Ida"`, `group="red"`, `rank="1"` |
| 3 | Person | `name="Sol"`, `active=true` |
| 4 | Place | `name="Stair Garden"` |
| 5 | Place | `name="Lantern Pier"` |
| 6 | City | `name="Northbank"` |
| 7 | City | `name="Glass Bay"` |

| Edge ID | From → To | Type |
| --- | --- | --- |
| 10 | 1 → 4 | VISITED |
| 11 | 1 → 4 | VISITED |
| 12 | 1 → 5 | VISITED |
| 13 | 2 → 4 | VISITED |
| 14 | 3 → 5 | VISITED |
| 20 | 4 → 6 | IN |
| 21 | 5 → 7 | IN |
| 22 | 1 → 1 | KNOWS |
| 23 | 1 → 2 | KNOWS |

- `MATCH (a:Person)-[:VISITED]->(b:Place) WHERE a.group = "blue" RETURN a,b` returns ordered path tuples `[1,10,4], [1,11,4], [1,12,5]`. With `LIMIT 2` it returns the first two tuples.
- `MATCH (a:Person)-[:VISITED]->(b:Place)-[:IN]->(c:City) WHERE a.rank = 1 RETURN a,b,c` returns `[1,10,4,20,6], [1,11,4,20,6], [1,12,5,21,7]`. Change the predicate to `a.rank = "1"` and the only row is `[2,13,4,20,6]`. Filtering on missing predicate key `unknown` returns zero rows.
- `MATCH (a:Place)<-[:VISITED]-(b:Person) RETURN a,b` returns `[4,10,1], [4,11,1], [4,13,2], [5,12,1], [5,14,3]`. Query two outward KNOWS hops from blue returns exactly `[1,22,1,22,1], [1,22,1,23,2]`.
- The second batch adds Person 8 with `group="blue"` and edge 30 from 8 to Place 4. The first query gains `[8,30,4]` as its final row after restart. A variant with a repeated key or edge to unknown node 99 fails as a whole and leaves the successful rows unchanged.
- Generate a separate deterministic fixture with Person IDs 1 through 1,024, Place 2,000 and one VISITED edge from each person with ID equal to the person ID. Only Person 417 has blue group. If testing through import, load Place 2,000 and the first 999 person records together, the remaining 25 persons next, then the edges in two batches of 1,000 and 24. A blue query returns `[417,417,2000]` with one visited start candidate under the index and 1,024 under forced scan. Both visit the single adjacency candidate reached after filtering.

## Epic E75 — Load, query and recover a personal graph offline

Exit once the browser slice, bounded traversal, durable import, scan/index evidence, readable query results and clean-checkout verification have passed. Estimates include implementation, tests and review. **Total effort is 17–27 engineering hours.** Work starts with E75-01. Split E75-02 and E75-03 after its graph and query response contract is committed. Then split E75-04 and E75-05. Bring both into E75-06. Main dependency chain is E75-01 → {E75-02, E75-03} → {E75-04, E75-05} → E75-06. E75-04 owns the engine diagnostics contract and E75-05 can build against fixed sample diagnostic responses until that contract is wired.

### E75-01 — Serve the first end-to-end graph query

**Estimate 3–4 h. Dependencies none.** Set up the std-only crate, typed graph representation and original fixture. Create demo bootstrap, the smallest browser surface and the one-hop outgoing path used by the first sample. Fix stable graph IDs and response fields in the module contracts for subsequent tickets.

- The documented demo starts on 127.0.0.1. The preset query returns the three exact blue paths, including both parallel edges between nodes 1 and 4. The browser shows their three rows and highlights the corresponding graph edges.
- Add data-contract tests for separate node and edge ID namespaces and the fixed shape of all first-slice success and error responses. Reject a malformed request without affecting the process.
- Verify with `cargo test --frozen` and a browser run against the bundled sample using the installed toolchain offline.

### E75-02 — Finish bounded traversal and deterministic semantics

**Estimate 3–5 h. Depends on E75-01. May run beside E75-03.** Own the tokenizer, parser, query model, exhaustive label-scan engine and reference graph enumeration test harness. Keep parsing and traversal separate from HTTP, import and storage.

- Cover both directions, two hops, self-loops, reused edges, duplicate paths by edge identity, typed comparison, missing values, numeric order and LIMIT truncation. Match every applicable fixture tuple above in exact order.
- Report stable error codes and byte offsets for malformed input, including oversized queries, unknown escapes, unsupported aliases, out-of-range LIMIT and trailing tokens. Force the traversal budget in a boundary test and assert `QUERY_LIMIT` with no partial rows.
- For tiny generated graphs, independently enumerate paths with direct joins over raw edge rows and endpoint lookups, with no production parser, adjacency or sort helper. Compare expected tuples from structured cases to the production query engine.

### E75-03 — Commit whole batches and verify journal recovery

**Estimate 4–6 h. Depends on E75-01. May run beside E75-02.** Own TSV input, graph validation, Linux file locks, versioned journal, recovery and restart. Validate candidate changes before appending. Exercise valid node-only and edge-only batches as well as mixed batches.

- Import a valid second batch. Restart against the same journal and recover the added row exactly once. Reject reused IDs, duplicate property keys, dangling edges, malformed types, oversized batches and lock contention without appending data.
- Prove with temporary state that EOF at each byte in a final frame exposes only earlier valid transactions. Separately corrupt a complete frame's size check or CRC and assert replay fails, with no auto-repair. Exercise the writer's valid-boundary truncation and repair followed by a fresh complete append.
- Verify a known CRC vector, `123456789` → `0xCBF43926`, and check that success is emitted only after the sync call. Inject an append/sync failure in the store harness. Verify that the CLI reports an indeterminate commit and recovery uses validated on-disk frames. Verify creation and second-writer contention on Linux.

### E75-04 — Add the derived index and verifiable plans

**Estimate 3–5 h. Depends on E75-02 and E75-03. May run beside E75-05.** Own typed index construction, eligibility, plan selection and execution counters. Record stable diagnostics in library and CLI. Extend the independent enumeration tests to both access paths.

- The selective fixture produces one candidate start inspection versus 1,024 and exactly the same ordered rows and adjacency visit count. Check forced-scan and no-predicate cases, equal-cardinality fallback, different typed keys and a missing indexed value with zero candidates.
- Reopen the changed journal from E75-03 and prove the index is rebuilt with the new exact results. Compare indexed and forced scan outputs to the independent enumerator over deterministic tiny graph cases with both directions, two hops, parallel edges, no results and LIMIT.
- `EXPLAIN` completes on a selective query without executing adjacency traversal. Tests verify plan choice and advertised estimates, and actual work is verified with explicit counters in a separate execution.

### E75-05 — Make graph paths, errors and plans usable in the browser

**Estimate 2–4 h. Depends on E75-02 and E75-03. May run beside E75-04.** Own the browser result table, path preview and diagnostic UI. Build on the E75-01 contract and use sample diagnostic responses during parallel work. E75-06 performs the live cross-check.

- The query editor supports presets, keyboard submission, visible focus and a status region for success or error. A successful two-hop query displays distinct edge IDs and a textual equivalent of every visual path. A parse error displays its byte offset and preserves the query. A successfully executed zero-row query clears previous results.
- Show the scan/index decision and execution counts for query responses using their documented names. Graph overflow shows the clipping status while the returned rows remain available in the table. A node name with markup-like characters renders verbatim as text.
- Verify in an ordinary browser against loopback. Inspect malformed, over-limit, empty and multi-edge results. Confirm that requests go exclusively to the local process.

### E75-06 — Finish the offline, restart and regression proof

**Estimate 2–3 h. Depends on E75-04 and E75-05.** Own clean-checkout instructions and the final CLI/browser verification. Fold results back into the epic and document operating limits and errors.

- Document installed-toolchain preflight and run `cargo fmt --check`, `cargo test --frozen` and the bundled demo from a clean checkout with downloads unavailable. Confirm the lockfile lists only the workbench package, the UI loads without internet resources and the loopback server never binds on every interface.
- Run `init`, a valid `load`, CLI query and `serve` with a fresh Linux temporary database. Add the second batch, restart and verify exact expected rows in both interfaces. Record clean handling of a recoverable tail, complete-frame corruption and concurrent writer contention using disposable copies.
- Record actual command output and the resulting pass or fail evidence. Check the selective fixture counters in the CLI and the plan and counts presented by the browser. Confirm a run can complete with the installed compiler and no registry mirror.

## Scope decisions and risks

The single-process design keeps the first run simple and the graph inspectable. Flat-file replay trades startup and size headroom for a small storage design that can be verified. The Linux filesystem and bounded fixtures are part of the release contract. Host-mounted Windows disks, external journal writers and real power loss would need separate verification. Conservative plan diagnostics and a small explicit language keep correctness testable against manually computed results.
