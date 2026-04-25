# Scan Directories (scan)

`woof scan` performs a targeted, isolated secret scanning sweep on a specific directory or file. Unlike `audit` and `patrol`, it **does not** evaluate lockfiles or communicate with the OSV API.

## Execute a Targeted Scan

=== "Scan Directory"
    ```bash {.mac-terminal}
    woof scan ./src/backend
    ```
=== "Scan Specific File"
    ```bash {.mac-terminal}
    woof scan ./config/production.json
    ```

## Apply Scan-Specific Flags

| Flag | Description |
| :--- | :--- |
| `--no-entropy` | Disables the Shannon Entropy mathematical context filter. This increases speed slightly but will result in significantly higher false positives. |
| `--max-file-size <BYTES>` | Overrides the strict OOM-immune file size limit for this specific scan (e.g., `--max-file-size 50MB`). |

!!! info "Use Case"
    Use `woof scan` when you need to quickly check an isolated payload, an external directory, or a single file without triggering the overhead of a full project and dependency audit.
