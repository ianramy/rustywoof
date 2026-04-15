# Secret Scanning Engine

At the heart of Rustywoof is a text-processing engine built for speed and accuracy. Scanning massive monorepos for thousands of distinct secret patterns can easily cause traditional CI tools to bottleneck or run out of memory (OOM).

Rustywoof solves this using a two-pass detection phase and strict memory constraints.

## 1. The Aho-Corasick Automaton

Instead of evaluating regex patterns one by one (which scales poorly as you add custom rules), Rustywoof compiles all literal prefixes into an **Aho-Corasick** automaton.

This algorithm searches for multiple string patterns simultaneously. The time complexity is exactly $O(n + m + z)$, where $n$ is the length of the file being scanned, $m$ is the total length of the search patterns, and $z$ is the number of matches.

> [!NOTE]
> Because the search time depends primarily on the length of the text rather than the number of patterns, you can add hundreds of custom domain-specific rules to your `.woof.toml` without degrading Rustywoof's performance.

## 2. Mathematical Context

Regex alone is prone to false positives (e.g., catching dummy test tokens or long variables). To filter these out, Rustywoof subjects potential matches to a mathematical threshold by calculating their **Shannon Entropy**.

Entropy measures the unpredictability or randomness of a string. High-entropy strings are likely to be cryptographic keys or hashes, while low-entropy strings are likely human-readable words.

The engine calculates the entropy $H(X)$ of a suspected secret using the formula:

```rs
$$H(X) = - \sum_{i=1}^{n} P(x_i) \log_2 P(x_i)$$
```

Where $P(x_i)$ is the probability of a given character appearing in the string. If a matched string's entropy falls below the configured threshold, Rustywoof flags it as a false positive and silently drops it.

## 3. OOM-Immunity

To ensure maximum stability on heavily constrained CI/CD runners, the scanning engine employs proactive safety checks:

- **Binary Bypassing:** Rustywoof detects and immediately skips compiled binaries, images, and other non-text artifacts.
- **Strict Size Constraints:** Files exceeding a hardcoded threshold (or a custom threshold set in `.woof.toml`) are skipped, preventing the engine from attempting to load massive log files or uncompressed database dumps into memory.

> [!WARNING]
> By default, Rustywoof ignores files larger than 10MB. You can override this behavior using the `--max-file-size` flag, but doing so increases the memory footprint.
