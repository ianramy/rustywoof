# Remediate Vulnerabilities (remediate)

When `woof audit` or `woof patrol` uncovers a compromised dependency, `woof remediate` acts as your automated response tool. It forces your local package manager to upgrade the compromised package to the nearest secure version.

## Execute Automated Remediation

```bash {.mac-terminal}
woof remediate <package_name> <current_version>
```

### Parameters

| Parameter | Description |
| :--- | :--- |
| `<package_name>` | The exact name of the compromised package (e.g., `axios`, `requests`). |
| `<current_version>` | The vulnerable version currently locked in your project (e.g., `1.7.4`). |

!!! example "Practical Example"
    If an audit alerts you that `axios` version `1.7.4` has a known vulnerability:
    ```bash {.mac-terminal}
    woof remediate axios 1.7.4
    ```

## Understand the Patching Logic

Rustywoof identifies the package ecosystem (e.g., detecting Node.js via `package.json`), queries the registry for the patched version defined in the OSV advisory, and executes the native upgrade command (e.g., `npm install axios@^1.7.5`) under the hood.

???+ tip "Semantic Versioning Safety"
    Rustywoof attempts to apply the least-disruptive patch. It heavily prefers minor or patch updates over major version bumps to maintain backward compatibility and avoid breaking your existing codebase.
