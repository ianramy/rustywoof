# Patrol Network Rules (patrol)

`woof patrol` is the flagship command of Rustywoof. It executes a full perimeter sweep of your current working directory. 

This command is the equivalent of running a secret scan and a dependency audit simultaneously. It is highly recommended for local, full-repository health checks.

## Execute the Patrol

```bash {.mac-terminal}
woof patrol
```

## Understand the Execution Matrix

1. :material-regex: **Secret Sweep:** Recursively scans the current directory using the Aho-Corasick engine, obeying all rules and ignore paths in your `woof.toml`.
2. :material-wall: **Perimeter Check:** Validates that no `.env` or sensitive identity files (like `.pem` or `id_rsa`) are staged without proper `.gitignore` coverage.
3. :material-link-lock: **Dependency Audit:** Locates any supported lockfiles and batches them to the OSV API for zero-day vulnerability detection.

!!! info "Concurrency Model"
    `patrol` is heavily multi-threaded. The CPU-bound secret scanning and the network-bound OSV audits run concurrently to ensure the command finishes in milliseconds.
