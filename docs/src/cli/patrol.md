# `woof patrol`

`woof patrol` is the flagship command of Rustywoof. It executes a full perimeter sweep of your current working directory.

This command is the equivalent of running a secret scan and a dependency audit simultaneously. It is the recommended command to run in your GitHub Actions, GitLab CI, or Jenkins pipelines.

### Usage

```bash
woof patrol
```

**What it does**

1. Secret Sweep: Recursively scans the current directory using the Aho-Corasick engine, obeying all rules and ignore paths in `.woof.toml`.

2. Perimeter Check: Validates that no `.env` or sensitive identity files are staged without `.gitignore` coverage.

3. Dependency Audit: Locates any supported lockfiles (`Cargo.lock`, `package-lock.json`, etc.) and batches them to the OSV API for vulnerability detection.

> [!NOTE]
> `patrol` is heavily multi-threaded. The secret scanning and network-bound OSV audits run concurrently to ensure the command finishes in milliseconds.
