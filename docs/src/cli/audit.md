# `woof audit`

`woof audit` strictly evaluates your project's dependencies against live threat intelligence from the Open Source Vulnerability (OSV) database.

### Usage

```bash
woof audit
```

**What it does**

- Crawls your project for known package manager lockfiles.

- Extracts the exact resolved versions of every dependency.

- Batches the metadata into a single JSON payload and queries the OSV API.

- Outputs a report detailing any known CVEs or GitHub Security Advisories associated with your dependencies.

> [!WARNING]
> This command requires an active internet connection. If the OSV API is unreachable, the command will exit with an error to prevent a false sense of security (fail-closed).
