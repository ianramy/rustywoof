# Perimeter Defense

The most effective way to remediate a leaked secret or a compromised dependency is to ensure it never leaves the developer's workstation. 

Rustywoof embraces a strict "shift-left" security model. By integrating directly into your version control workflow, it acts as an automated perimeter guard, catching vulnerabilities at the exact moment a developer attempts to commit them.

## Deploy the Pre-Commit Guard

The cornerstone of Rustywoof's perimeter defense is its native Git integration. By executing the hook installation, Rustywoof automatically deploys a highly optimized Git `pre-commit` hook into your local repository.

Every time a developer runs `git commit`, Rustywoof intercepts the staged changes and performs a micro-sweep of the diff.

* :material-lightning-bolt: **Achieve Lightning Speed:** Because the Aho-Corasick engine is $O(n)$ and only scans the staged diff (not the entire project), the pre-commit check typically executes in under 10 milliseconds.
* :material-brain: **Leverage Context:** The hook knows exactly which files are being modified and applies your custom `woof.toml` ignore paths automatically.

!!! warning "Commit Rejection Protocols"
    If a developer attempts to commit a flagged cryptographic credential, Rustywoof will **hard reject the commit**. It immediately prints the exact file, line number, and regex rule that triggered the block to the terminal stdout.

### Bypass the Guard (Emergencies Only)

In rare emergency scenarios—or when explicitly handling a false positive that hasn't been added to the ignore list yet—developers can bypass the guard using Git's native override mechanism:

```bash {.mac-terminal}
git commit -m "Emergency hotfix" --no-verify # (1)
```

1. :material-alert: **Audit Trail:** While bypassing is possible locally, `woof patrol` should still be running in your CI/CD pipeline. The pipeline will catch and flag any forced commits that bypassed the local pre-commit hook.

## Detect Un-tracked Environment Files

One of the most common vectors for massive credential leaks is the accidental staging of environment files. A developer creates a `.env` file for local testing, forgets to add it to `.gitignore`, and carelessly runs `git add .`.

Rustywoof deploys dedicated heuristics specifically for `.env`, `.pem`, and `id_rsa` files.

During a pre-commit sweep, Rustywoof doesn't just scan the contents of these files—it checks their Git tracking status. If Rustywoof detects a high-risk file extension that is staged for commit or lacks `.gitignore` coverage, it throws a critical perimeter alert **even if the file currently contains no known secret patterns**.

## Distribute Fleet-Wide

For enterprise teams, relying on developers to manually run local installation commands is a severe compliance risk. Rustywoof is designed to be easily distributed fleet-wide using centralized tools.

=== ":material-language-python: Pre-Commit Framework"
    Integrate via `.pre-commit-config.yaml` at the root of your repository.
    ```yaml
    repos:
        - repo: https://github.com/ianramy/rustywoof
        rev: v0.1.8
        hooks:
            - id: rustywoof
    ```

=== ":material-nodejs: Husky (Node.js)"
    Add the execution script to your `.husky/pre-commit` file.
    ```bash {.mac-terminal}
    #!/usr/bin/env sh
    . "$(dirname -- "$0")/_/husky.sh"

    npx @ianramy/rustywoof scan --staged
    ```
