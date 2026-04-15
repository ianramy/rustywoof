# `woof scan`

`woof scan` performs a targeted secret scanning sweep on a specific directory or file. Unlike `audit` & `patrol`, it **_does not_** audit lockfiles or communicate with the OSV API.

### Usage

```bash
# Scan a specific directory
woof scan ./src/backend

# Scan a single file
woof scan ./config/production.json
```

### Common Flags

`--no-entropy`: Disables the Shannon Entropy mathematical context filter. This will increase speed slightly but may result in higher false positives.

`--max-file-size <BYTES>`: Overrides the strict OOM-immune file size limit for this specific scan.

> [!TIP]
> Use `woof scan` when you need to quickly check an isolated payload or directory without triggering a full project audit.
