# CLI Reference

The `woof` command-line interface is designed to be simple, predictable, and highly composable for CI/CD pipelines.

Every command in Rustywoof is built around a specific security domain: scanning code, auditing dependencies, or managing Git hooks.

### Global Flags

The following flags can be appended to any `woof` command:

- `-h, --help`: Prints help information for the specific command.
- `-V, --version`: Prints the current Rustywoof version.
- `-v, --verbose`: Enables debug-level logging. Useful for troubleshooting why a specific file was ignored or tracing OSV API network requests.
- `-c, --config <PATH>`: Overrides the default `.woof.toml` configuration path.

> [!TIP]
> **Exit Codes:** Rustywoof follows standard POSIX exit codes. An exit code of `0` means the sweep passed with zero vulnerabilities. An exit code of `1` means one or more vulnerabilities (leaked secrets or compromised dependencies) were found, which will intentionally fail your CI/CD pipeline.
