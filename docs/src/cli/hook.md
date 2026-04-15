# `woof hook`

The `woof hook` command manages Rustywoof's native Git integration, providing proactive perimeter defense at the developer's workstation.

### Usage

```bash
# Deploy the pre-commit guard
woof hook install

# Remove the pre-commit guard
woof hook remove
```

**What install does**
It creates or modifies the .git/hooks/pre-commit file in your local repository. Once installed, every git commit command will trigger a lightning-fast micro-sweep of your staged files. If a leaked secret is detected, the commit is instantly aborted.

> [!WARNING]
> This command only installs the hook in your local repository clone. To enforce this across your entire team automatically, consider using a centralized tool like Husky or the Python pre-commit framework to run woof scan against staged files.
