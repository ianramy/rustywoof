# Core Concepts

Rustywoof is built on three foundational security pillars. Rather than relying on a single method of detection, it uses a layered architecture to secure both your codebase and your dependency tree.

Understanding how these systems work under the hood will help you configure Rustywoof for maximum efficiency in your specific environment.

### The Three Pillars

1. **[Secret Scanning Engine](./secret-scanning.md):** The high-speed text analysis layer. It combines deterministic finite automatons with information theory to find credentials without bringing down your CI runner.
2. **[Supply Chain Watchdog](./supply-chain.md):** The dependency analysis layer. It actively queries live threat intelligence to catch compromised upstream packages before they are merged.
3. **[Perimeter Defense](./perimeter-defense.md):** The proactive barrier. It integrates deeply with Git to stop vulnerabilities at the developer's workstation.

Choose a topic above to explore the internal mechanics of Rustywoof.
