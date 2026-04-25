# Audit Environments (audit)

`woof audit` strictly evaluates your project's dependencies against live threat intelligence from the Open Source Vulnerability (OSV) database.

## Execute the Audit

```bash {.mac-terminal}
woof audit
```

## Review the Execution Flow

1. :material-file-find: **Crawls** your project for known package manager lockfiles (`Cargo.lock`, `package-lock.json`, etc.).
2. :material-extract: **Extracts** the exact, resolved versions of every dependency.
3. :material-network: **Batches** the metadata into a single, highly-optimized JSON payload and queries the OSV API.
4. :material-text-box-search: **Outputs** a report detailing any known CVEs or GitHub Security Advisories associated with your supply chain.

???+ warning "Fail-Closed Security Posture"

    This command requires an active internet connection to ensure threat data is strictly real-time. If the OSV API is unreachable, the command will exit with an error code of `1` to prevent a false sense of security (a fail-closed design).