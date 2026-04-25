# Update Definitions (update)

Security tools must stay up-to-date to remain effective against newly discovered vulnerabilities. The `woof update` command is a self-updating mechanism that upgrades your Rustywoof engine to the latest stable release.

## Execute the Update

```bash {.mac-terminal}
woof update
```

## Review the Update Pipeline

Rather than relying on your system's package manager (which may have outdated caches), `woof update` performs a direct check against the official Rustywoof GitHub release registry.

1. :material-cpu-64-bit: **Architecture Resolution:** Detects your host operating system and CPU architecture (e.g., `x86_64-unknown-linux-gnu` or `aarch64-apple-darwin`).
2. :material-source-commit: **Version Comparison:** Compares your local binary version against the latest tagged release payload.
3. :material-swap-horizontal: **In-Place Upgrade:** If an update is available, it downloads the correct pre-compiled binary and seamlessly replaces your current executable.

!!! tip "CI/CD Stability"
    While `woof update` is excellent for local developer machines, we highly recommend pinning Rustywoof to a specific version in your CI/CD pipelines to guarantee reproducible, immutable builds.