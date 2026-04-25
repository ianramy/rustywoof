# Check Dependencies & Perimeters (check)

While commands like `patrol` and `scan` are great for interactive use on a developer's local machine, `woof check` is strictly designed for automation. 

It is the designated CI/CD enforcer for Rustywoof, executing a rigorous security sweep while optimizing its output for headless runner environments.

## Execute the CI Sweep

```bash {.mac-terminal}
woof check
```

## Compare Interactive vs. Automated Modes

| Feature | `scan` / `patrol` | `check` (Automated) |
| :--- | :--- | :--- |
| **Exit Codes** | Warnings may not fail the process | Strict `1` exit code instantly on any confirmation |
| **Console Output** | Animated progress bars, rich color | Suppressed animations; raw log-friendly formatting |
| **Network Failures** | Prompts user or skips gracefully | Hard fails the build (Fail-Closed Architecture) |
| **Untracked `.env`** | Throws a warning | Fails the build immediately |

???+ warning "Strict Compliance"
    Because `woof check` is designed for strict compliance, it will fail the build if it detects an untracked `.env` file, even if that file does not currently contain a known secret pattern. Exposing an environment file is fundamentally a perimeter breach.

## Integrate via GitHub Actions

For maximum security, run `woof check` on every Pull Request before code merges into your main branch.

```yaml
name: Rustywoof Security Sweep

on:
    push:
    branches: [ "main" ]
    pull_request:
    branches: [ "main" ]

jobs:
    security-check:
    name: Perimeter Defense
    runs-on: ubuntu-latest
    steps:
        - name: Checkout Code
        uses: actions/checkout@v4

        - name: Install Rustywoof # (1)
        run: |
            curl -sSL https://ianramy.co.ke/rustywoof/installer.sh | bash

        - name: Execute Strict CI Sweep # (2)
        run: woof check
```

1. :material-download: Pulls the latest binary designed for the Ubuntu CI runner natively.
2. :material-shield-check: Executes the non-interactive, fail-closed security sweep.
