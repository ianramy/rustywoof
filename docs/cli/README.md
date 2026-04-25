# Command Reference

The `woof` command-line interface is designed to be simple, predictable, and highly composable for CI/CD pipelines. Every command in Rustywoof is built around a specific security domain: scanning code, auditing dependencies, or managing Git hooks.

## Apply Global Flags

The following flags can be appended to any `woof` command to modify its baseline execution context.

| Flag | Long Flag | Description |
| :--- | :--- | :--- |
| `-h` | `--help` | Prints help information for the specific command. |
| `-V` | `--version` | Prints the current Rustywoof version. |
| `-v` | `--verbose` | Enables debug-level logging. Useful for troubleshooting ignored files or tracing OSV API network requests. |
| `-c` | `--config <PATH>` | Overrides the default `woof.toml` configuration path. |

!!! tip "Standardized Exit Codes"
    Rustywoof strictly follows standard POSIX exit codes. An exit code of `0` means the sweep passed with zero vulnerabilities. An exit code of `1` means one or more vulnerabilities (leaked secrets or compromised dependencies) were found, which will intentionally and correctly fail your CI/CD pipeline.
