# `woof check`

While commands like `patrol` and `scan` are great for interactive use on a developer's local machine, `woof check` is strictly designed for automation.

It is the designated CI/CD enforcer for Rustywoof, executing a rigorous security sweep while optimizing its output for headless environments.

### Usage

```bash
woof check
```

**How it differs from interactive commands**

Strict Exit Codes: woof check guarantees a POSIX-compliant exit code of 1 the millisecond a leaked secret or compromised dependency is confirmed, instantly failing your pipeline.

CI-Optimized Output: It suppresses interactive terminal features (like animated progress bars or color codes that don't render well in logs) to keep your CI output clean, readable, and easy to parse.

Fail-Closed Architecture: If woof check cannot reach the OSV API for the dependency audit, it will fail the build rather than silently skipping the audit. In security, no result is a failure.

> [!WARNING]
> Because woof check is designed for strict compliance, it will also fail the build if it detects an untracked .env file, even if that file does not currently contain a known secret pattern.

Integration Example: GitHub Actions
For maximum security, you should run woof check on every Pull Request before code is allowed to merge into your main branch.

Here is a drop-in GitHub Actions workflow to integrate Rustywoof into your repository:

```YAML
name: Rustywoof Security Sweep

on:
push:
branches: [ "main" ]
pull_request:
branches: [ "main" ]

jobs:
security-check:
name: Perimeter Defense
runs-on: ubuntu-latest
steps: - name: Checkout Code
uses: actions/checkout@v4

      - name: Install Rustywoof
        run: |
          curl --proto '=https' --tlsv1.2 -LsSf [https://github.com/ianramy/rustywoof/releases/latest/download/installer.sh](https://github.com/ianramy/rustywoof/releases/latest/download/installer.sh) | sh

      - name: Execute Strict CI Sweep
        run: woof check

```

> [!TIP]
> Performance: Because Rustywoof is written in Rust and utilizes Aho-Corasick with JSON batching, this entire GitHub Action job (including installation) typically completes in under 3 seconds.
