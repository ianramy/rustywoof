<div align="center">
  <img src="https://www.ianramy.co.ke/rustywoof/assets/logo.png" width="250" />
  <h1 align="center">
   Rustywoof
  </h1>
  <br>

  [![Build](https://img.shields.io/badge/build-Passing-green.svg?style=flat-square&logo=rust)]()
  [![Version](https://img.shields.io/badge/version-v0.2.4-orange.svg?style=flat-square&logo=rust)]()
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
  [![Category](https://img.shields.io/badge/category-Cybersecurity-red.svg)]()
  [![Integration](https://img.shields.io/badge/integration-OSV_Database-purple.svg)](https://osv.dev/)
  [![Status](https://img.shields.io/badge/status-Open_Source-brightgreen.svg?style=flat-square&logo=github)]()
  [![Docs](https://img.shields.io/badge/docs-Available-blue.svg?style=flat-square&logo=readthedocs)]()
</div>

**Enterprise-grade perimeter defense, secret scanner, and supply chain watchdog.**

Rustywoof is a high-performance, memory-safe command-line engine designed to detect exposed cryptographic credentials and compromised dependencies before they breach your system perimeter. Built entirely in Rust, it is engineered for strict enterprise CI/CD environments, utilizing zero-copy memory mapping and an O(n) Aho-Corasick automaton to analyze thousands of files in milliseconds without exhausting system resources.

---

## Security & Trust Posture

We recognize that running a security binary locally requires absolute trust. Rustywoof is built with strict operational transparency:

* **No Telemetry or Exfiltration:** The engine does not track user data, usage analytics, or exfiltrate source code.

* **Local Execution:** Secret scanning operations (Aho-Corasick pre-filtering and Regex validation) are executed entirely on your local machine.

* **Verifiable Outbound Traffic:** The only outbound network requests made by Rustywoof are standard HTTPS POST requests directed exclusively to [https://api.osv.dev](https://api.osv.dev) (Google's Open Source Vulnerability database) during the audit or patrol commands. 

* **Open Source Verifiability:** The complete source code is available for independent audit. We encourage security researchers and engineers to compile the binary directly from the source.

---

## Core Capabilities

### 1. High-Performance Secret Detection

* **Aho-Corasick Automaton:** Utilizes simultaneous, multi-pattern literal prefix matching to filter text in O(n) time before invoking expensive Regex evaluations.

* **Zero-Copy Architecture:** Employs OS-level memory mapping (mmap) to read files directly as byte slices, drastically reducing RAM allocation.

* **Binary Fast-Fail Heuristic:** Instantly identifies and bypasses compiled binaries by evaluating the initial byte headers, preventing Out-Of-Memory (OOM) crashes on large artifacts.

* **Cryptographic Context:** Calculates the Shannon Entropy of matched strings to confidently differentiate between genuine cryptographic keys and high-entropy plaintext, heavily reducing false positives.

### 2. Supply Chain Threat Intelligence

* **OSV Batch Querying:** Parses multiple lockfiles (Cargo, npm, pnpm, yarn, Poetry, pip) and audits them against the Open Source Vulnerability (OSV) database. (Note: This specific feature requires an active internet connection to query the upstream database).
* **Local Threat Caching:** To prevent network bottlenecks during repeated CI/CD runs, Rustywoof caches OSV API responses locally with a 12-hour Time-To-Live (TTL).
* **Dependency Sniffing:** Provides deep visibility into your dependency graph, tracing paths from workspace roots to specific vulnerable packages.
* **Automated Remediation:** Capable of spawning shell processes to force native package managers to update a compromised dependency to a secure target version.

### 3. Proactive Perimeter Defense

* **Staged-Only Git Hooks:** Manages pre-commit hooks that strictly scan newly staged files (git diff --cached), ensuring rapid execution without scanning the entire repository.
* **Environment Guarding:** Automatically detects untracked .env files and interactively deploys strict .gitignore configurations to prevent accidental credential leakage.

---

## Installation

### Option 1: Quick Install (macOS / Linux)

To install Rustywoof on macOS or Linux, run the following command in your terminal:

```bash
curl -sSL https://ianramy.co.ke/rustywoof/installer.sh | sh
```

### Option 2: Windows (PowerShell)

To install Rustywoof on Windows, run the following command in your PowerShell terminal:

```bash
irm https://ianramy.co.ke/rustywoof/installer.ps1 | iex
```

### Option 3: Compile from Source (Via Cargo)

To compile the engine from source, use Cargo, Rust's package manager.

```bash
cargo install rustywoof
```

> [!NOTE]
> For the highest level of trust, use `cargo binstall` to compile the engine from source.

```bash
cargo binstall rustywoof
```


### Option 4: Node.js Package Managers

To install Rustywoof via Node.js package managers, use the following commands:

```bash
npm install -g @ianramy/rustywoof
```

```bash
pnpm install -g @ianramy/rustywoof
```

```bash
yarn global add @ianramy/rustywoof
```

```bash
bun install -g @ianramy/rustywoof
```

---

## Usage Guide

Rustywoof is designed to provide maximum visibility with minimal configuration.

```bash
# Execute a full perimeter sweep (Code Secrets + Dependency Threats)
woof patrol

# Scan a specific directory or file for exposed secrets
woof scan ./src

# Audit project lockfiles against the OSV vulnerability database
woof audit

# Trace dependency paths leading to a specific package
woof sniff <packagename>

# Force a package manager to remediate a compromised dependency
woof remediate <packagename> <version>

# Purge the local threat intelligence cache
woof cache clean

# Deploy the Watchdog pre-commit guard to block leaked secrets natively in Git
woof hook install

# For help with available commands and options
woof help

# For version information
woof version
```

---

## Shell Completions

Rustywoof supports native auto-completion for your favorite shell, making it easier to discover commands and flags on the fly. 

To enable tab completions, run the `generate` command for your specific shell and pipe the output to your shell's completion directory.

**For Fish:**
```bash
woof generate fish > ~/.config/fish/completions/woof.fish
```

**For Zsh:**
```bash
# Ensure ~/.zfunc is added to your $fpath in .zshrc
woof generate zsh > ~/.zfunc/_woof
```

**For Bash:**
```bash
woof generate bash > ~/.local/share/bash-completion/completions/woof
```

**For PowerShell:**
```powershell
woof generate powershell > $PROFILE\..\woof-completions.ps1
# Then add `. $PROFILE\..\woof-completions.ps1` to your PowerShell profile
```

----

## Configuration

Run `woof init` to generate a `.woof.toml` configuration file in your project root. This allows teams to define domain-specific rules, suppress false positives via entropy tuning, and manage scan perimeters.

```toml
# Directories to bypass during sweeps
ignore_paths = ["tests/", "node_modules/", "target/"]

# Adjust the Shannon entropy threshold (lower values increase sensitivity)
min_entropy = 3.0

# Define proprietary token formats
[[custom_rules]]
name = "Internal API Token"
pattern = "api_prod_[a-zA-Z0-9]{32}"
```

> [!NOTE]
> Developers can also use inline suppression directives like `// woof:ignore-next-line` directly in their source code to bypass specific flagged strings.

---

## License

This project is dual-licensed under either the [MIT license](LICENSE-MIT) or the [Apache License, Version 2.0](LICENSE-APACHE), at your option.
