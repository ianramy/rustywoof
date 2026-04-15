# Perimeter Defense

The most effective way to remediate a leaked secret or a compromised dependency is to ensure it never leaves the developer's workstation.

Rustywoof embraces a strict "shift-left" security model. By integrating directly into your version control workflow, it acts as an automated perimeter guard, catching vulnerabilities at the exact moment a developer attempts to commit them.

## 1. The Pre-Commit Guard

The cornerstone of Rustywoof's perimeter defense is its native Git integration. By running `woof hook install`, Rustywoof automatically deploys a highly optimized Git `pre-commit` hook into your local repository.

Every time a developer runs `git commit`, Rustywoof intercepts the staged changes and performs a micro-sweep of the diff.

- **Lightning Fast:** Because the Aho-Corasick engine is $O(n)$ and only scans the staged diff (not the entire project), the pre-commit check typically executes in under 10 milliseconds. Developers won't even notice it's there—until they make a mistake.
- **Context-Aware:** The hook knows exactly which files are being modified and applies your custom `.woof.toml` ignore paths automatically.

> [!WARNING]
> If a developer attempts to commit a flagged cryptographic credential, Rustywoof will **reject the commit** and print the exact file, line number, and regex rule that triggered the block.

### Bypassing the Guard

In rare emergency scenarios (or when explicitly handling a false positive that hasn't been added to the ignore list yet), developers can bypass the guard using Git's native override:

```bash
git commit -m "Emergency hotfix" --no-verify
```

_(Note: While bypassing is possible locally, `woof patrol` should still be running in your CI/CD pipeline to catch any forced commits.)_

## 2. Un-tracked _`.env`_ Detection

One of the most common vectors for massive credential leaks is the accidental staging of environment files. A developer creates a `.env` file for local testing, forgets to add it to `.gitignore`, and carelessly runs `git add .`.

Rustywoof has dedicated heuristics specifically for `.env`, `.pem`, and `id_rsa` files.

During a pre-commit sweep or a standard `woof patrol`, Rustywoof doesn't just scan the contents of these files—it checks their Git tracking status. If Rustywoof detects a high-risk file extension that is staged for commit or lacks `.gitignore` coverage, it will immediately throw a critical perimeter alert, even if the file currently contains no known secret patterns.

## 3. Fleet-Wide Deployment

For enterprise teams, relying on developers to manually run `woof hook install` is a compliance risk. Rustywoof is designed to be easily distributed fleet-wide.

> [!TIP]
> **Enterprise Strategy:** We recommend using tools like `pre-commit` (the Python framework) or Husky (Node.js) to manage your Git hooks centrally. Rustywoof exposes a lightweight `.pre-commit-hooks.yaml` definition, allowing you to enforce the Rustywoof guard across your entire organization simply by updating your repository's configuration.
