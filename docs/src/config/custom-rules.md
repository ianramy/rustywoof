# Custom Rules

While Rustywoof comes pre-loaded with heuristics for hundreds of standard cloud providers (AWS, GCP, Stripe, GitHub, etc.), enterprise codebases often have proprietary token formats.

You can instruct Rustywoof to hunt for your internal credentials by adding `[[custom_rules]]` blocks to your `.woof.toml`.

## Writing a Custom Rule

A custom rule requires a `name` for reporting and a `pattern` containing the literal prefix or regex to match.

```toml
[[custom_rules]]
name = "Internal API Token (Production)"
pattern = "api_prod_[a-zA-Z0-9]{32}"

[[custom_rules]]
name = "Legacy Database Password"
pattern = "db_pass_v1_[a-f0-9]{16}"
```

The Power of Aho-CorasickTraditional secret scanners loop through an array of regex rules, checking a file against Rule 1, then Rule 2, then Rule 3. This means adding 50 custom rules makes the scanner 50 times slower.Rustywoof does not do this. Thanks to the Aho-Corasick automaton, Rustywoof compiles your custom rules alongside its built-in rules into a single state machine. It evaluates the file once, searching for all patterns simultaneously.

> [!TIP]
> You can add hundreds of custom rules to your .woof.toml without degrading Rustywoof's runtime performance.
> The time complexity remains $O(n)$ based on the file size.Overriding Mathematical ContextBy default, any match found by your custom rule will be subjected to the global entropy_threshold defined in your configuration.However, if your internal tokens are structurally predictable (e.g., they contain lots of repeating characters or readable words), the entropy engine might flag them as false positives and ignore them.You can bypass the math engine for a specific rule by explicitly setting entropy = false:

```toml
[[custom_rules]]
name = "Low Entropy Internal Token"
pattern = "corp_token_dev_[a-z]{10}"
entropy = false # Bypasses the Shannon Entropy check
```

> [!WARNING]
> Use entropy = false sparingly. Disabling the mathematical context for generic or broad regex patterns will significantly increase the noise and false positives in your CI/CD pipeline.
