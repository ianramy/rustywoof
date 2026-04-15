# `woof remediate`

When `woof audit` or `woof patrol` uncovers a compromised dependency, `woof remediate` acts as your automated response tool. It forces your local package manager to upgrade the compromised package to the nearest non-vulnerable version.

### Usage

```bash
woof remediate <package_name> <current_version>
```

**Example**

If `woof audit` alerts you that `axios` version `1.7.4` has a known vulnerability:

```bash
woof remediate axios 1.7.4
```

**How it works**

Rustywoof identifies the package ecosystem (e.g., `Node.js` via `package.json`), queries the registry for the patched version mentioned in the OSV advisory, and executes the native upgrade command (e.g., `npm install axios@^1.7.5`) under the hood.

> [!NOTE]
> Rustywoof will attempt to apply the least-disruptive patch (preferring minor/patch updates over major version bumps) to maintain backward compatibility in your codebase.
