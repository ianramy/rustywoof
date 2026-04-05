# 🐕 Rustywoof (woof)

> **Enterprise-grade secret scanner and supply chain watchdog.**

Rustywoof is a blazing-fast, memory-safe CLI tool designed to detect exposed cryptographic credentials and compromised dependencies before they breach your perimeter. Built in Rust, it utilizes an `O(n)` Aho-Corasick automaton to scan thousands of files in milliseconds without crashing your CI/CD pipelines.

## ✨ Features

- **High-Speed Regex Pre-filtering:** Uses Aho-Corasick for simultaneous, multi-pattern literal prefix matching.
- **OOM-Immune Engine:** Strict file-size constraints and binary bypassing ensure stability on massive monorepos.
- **Live Threat Intelligence:** Queries the Open Source Vulnerability (OSV) API using JSON batching to audit Node, Rust, and Python lockfiles in under 500ms.
- **Proactive Perimeter Defense:** Automatically detects un-tracked `.env` files and manages Git `pre-commit` hooks.
- **Mathematical Context:** Calculates the Shannon Entropy of leaked credentials to reduce false positives.

## 🚀 Installation

### Option 1: Quick Install (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf [https://github.com/ianramy/rustywoof/releases/latest/download/installer.sh](https://github.com/ianramy/rustywoof/releases/latest/download/installer.sh) | sh
```

### Option 2: Via Cargo (Rust developers)

```bash
cargo install rustywoof

# or

cargo binstall rustywoof
```

### Option 3: Windows (PowerShell)

```bash
irm [https://github.com/ianramy/rustywoof/releases/latest/download/installer.ps1](https://github.com/ianramy/rustywoof/releases/latest/download/installer.ps1) | iex
```

## 🛠️ Usage

Rustywoof is designed to be simple and devastatingly effective.

```bash
# Execute a full perimeter sweep (Code + Dependencies)
woof patrol

# Scan a specific directory for secrets
woof scan ./src

# Audit lockfiles against the OSV vulnerability database
woof audit

# Force a package manager to remediate a compromised dependency
woof remediate axios 1.7.4

# Deploy the pre-commit guard to block leaked secrets
woof hook install
```

## ⚙️ Configuration

Run `woof init` to generate a `.woof.toml` file in your project root. You can add custom domain-specific regex rules and ignore paths here.

```toml
ignore_paths = ["tests/", "node_modules/", "target/"]

[[custom_rules]]
name = "Internal API Token"
pattern = "api_prod_[a-zA-Z0-9]{32}"
```

## 🛡️ License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.
