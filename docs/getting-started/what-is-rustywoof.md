# Understand Rustywoof
    
Rustywoof is engineered to solve the scaling problems inherent in modern DevSecOps. As monorepos grow, traditional secret scanners often bottleneck CI/CD pipelines, consuming excessive memory and triggering Out-Of-Memory (OOM) kills on CI runners.

## Compare the Ecosystem

When deciding on a security tool, it is critical to understand the architectural trade-offs. Here is how Rustywoof compares to industry standards.

| Feature / Tool | Rustywoof | Gitleaks | TruffleHog |
| :--- | :--- | :--- | :--- |
| **Core Algorithm** | Aho-Corasick Automaton + Regex | Pure Regex | Regex + Verifiers |
| **Memory Safety** | Strict constraints (OOM-immune) | Variable (Go garbage collection) | High memory overhead |
| **Dependency Auditing** | Native (OSV API Batching) | None | None |
| **Speed** | Blazing ($O(n)$ complexity) | Fast | Slower (due to active verification) |
| **False Positive Reduction** | Shannon Entropy Calculation | Context-based rules | Active API validation |

???+ info "The Aho-Corasick Advantage"

    Unlike pure regex engines that must evaluate every pattern against every string independently, Rustywoof uses an Aho-Corasick automaton. This pre-filtering mechanism allows for simultaneous, multi-pattern literal prefix matching. We only invoke expensive regex evaluations when a high-probability literal match is found, drastically reducing CPU cycles.

## Review Core Philosophies

1. **Fail-Fast Defense:** Detect vulnerabilities locally via Git pre-commit hooks before they enter the repository.
2. **Supply Chain Visibility:** Secrets are only half the battle. Auditing lockfiles (`Cargo.lock`, `package-lock.json`, `requirements.txt`) is built natively into the tool.
3. **Zero Configuration by Default:** Sane, secure defaults that work out-of-the-box, with the ability to override via `woof.toml` when needed.
