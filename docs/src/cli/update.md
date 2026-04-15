# `woof update`

Security tools must stay up-to-date to remain effective against newly discovered vulnerabilities. The `woof update` command is a self-updating mechanism that upgrades your Rustywoof engine to the latest stable release.

### Usage

```bash
woof update
```

**How it works**
Rather than relying on your system's package manager (which may have outdated caches), woof update performs a direct check against the official release registry.

Architecture Resolution: It detects your host operating system and CPU architecture (e.g., x86_64-unknown-linux-gnu for Linux, aarch64-apple-darwin for Apple Silicon).

Version Comparison: It compares your local binary version against the latest tagged release.

In-Place Upgrade: If an update is available, it downloads the correct pre-compiled binary and seamlessly replaces your current executable.

> [!NOTE]
> CI/CD Environments: While woof update is great for local developer machines, we recommend pinning Rustywoof to a specific version in your CI/CD pipelines (e.g., downloading v0.1.5 explicitly via the installation script) to ensure reproducible builds.
