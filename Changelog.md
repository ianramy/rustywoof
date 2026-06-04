# Release v0.2.0 - Performance & Precision Update - 04/06/2026

Rustywoof `v0.2.0` is a massive architectural upgrade focused on enterprise-grade performance, lockfile auditing accuracy, and improved developer experience. We've replaced heavy heap allocations with zero-copy memory mapping, moved to a lock-free concurrency model, and hardened our supply chain parsers.

## Features & Developer Experience
* **Configurable Cryptographic Sensitivity:** Added `min_entropy` to `.woof.toml`. Teams can now tune the Shannon entropy threshold to silence false positives (like dummy test tokens) while maintaining strict guards for production secrets.
* **Robust `requirements.txt` Parsing:** Replaced basic string splitting with a resilient Regex extractor. The engine now flawlessly handles environment markers, version ranges (`>=`, `~=`), and inline comments.
* **Graceful Background Updates:** Resolved the "Ghost Update" race condition. The background self-updater now synchronizes properly with the CLI execution thread, ensuring developers are reliably notified of new engine updates without hanging the terminal.

## Performance Optimizations
* **Zero-Copy Memory Mapping:** Replaced `fs::read_to_string` with OS-level memory mapping (`memmap2`). The Aho-Corasick automaton now scans raw `&[u8]` byte slices directly, drastically reducing RAM allocation and CPU overhead on large repositories.
* **Lock-Free Concurrency:** Eliminated the Mutex bottleneck (`Arc<Mutex<Vec>>`) that throttled multi-threaded execution. Findings are now transmitted instantly via a multi-producer single-consumer (`mpsc`) channel, vastly improving parallel scan times.
* **Binary Fast-Fail Heuristic:** The scanner now inspects the first 512 bytes of a file for null (`\0`) characters, instantly skipping compiled binaries and assets before attempting expensive UTF-8 validation.
* **Structured Deserialization:** Migrated lockfile parsing away from arbitrary DOM trees (`serde_json::Value`). By using strict Rust structs and ignoring unused fields, `package-lock.json` and `pnpm-lock.yaml` parsing times and memory footprint have been slashed.

## Bug Fixes
* **Git Hook Pathing Lockout:** Fixed a critical issue where the generated pre-commit hook would permanently block Git commits if the user did not have `woof` installed globally in their `PATH`. The hook now features a graceful fallback.
* **Directory Exclusion Traversals:** Corrected the implementation of the `ignore` crate. The scanner now correctly respects string paths defined in `.woof.toml` via `OverrideBuilder`, preventing unbounded traversals into `node_modules/` or `target/`.
* **Yarn Audit Blindspot:** Patched an audit/remediate mismatch. The engine now correctly parses `yarn.lock` during the OSV threat intelligence sweep.
* **pnpm Peer Dependency Parsing:** Fixed a string-splitting bug in the `pnpm-lock.yaml` parser that incorrectly captured peer dependencies as part of the SemVer string (e.g., `18.2.0(react@18.2.0)`), which previously caused OSV API query rejections.