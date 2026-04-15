# Introduction

Welcome to **Rustywoof** —an enterprise-grade secret scanner and supply chain watchdog.

Rustywoof is a blazing-fast, memory-safe CLI tool designed to detect exposed cryptographic credentials and compromised dependencies before they breach your perimeter. Built to handle massive monorepos, it ensures stability in your CI/CD pipelines without compromising on speed or depth of analysis.

> [!NOTE]
> **Why Rustywoof?** Under the hood, it utilizes an `$O(n)$` Aho-Corasick automaton to scan thousands of files in milliseconds. Combined with strict memory constraints and binary bypassing, it guarantees an OOM-immune execution environment.

### Core Capabilities

- **High-Speed Regex Pre-filtering:** Simultaneous, multi-pattern literal prefix matching.
- **Live Threat Intelligence:** Queries the Open Source Vulnerability (OSV) API using JSON batching to audit Node, Rust, and Python lockfiles in under 500ms.
- **Proactive Perimeter Defense:** Automatically detects un-tracked `.env` files and manages Git `pre-commit` hooks.
- **Mathematical Context:** Calculates the Shannon Entropy of leaked credentials to drastically reduce false positives.
