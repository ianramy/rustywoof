# Explore Features

Rustywoof exposes a robust suite of commands tailored for both local development and automated CI environments. 

Below is a summary of the core functionalities. For detailed parameter lists and configuration options, follow the links to the dedicated CLI documentation.

## Summarize Core Commands

| Command | Description | Deep Dive |
| :--- | :--- | :--- |
| `help` | Gives a summary of all the commands that can be excuted. | [Execute help](../cli/help.md) |
| `scan` | Sweeps directories or specific files for hardcoded secrets using the Aho-Corasick engine. | [Execute scan](../cli/scan.md) |
| `audit` | Evaluates project lockfiles against the OSV database to detect vulnerable dependencies. | [Execute audit](../cli/audit.md) |
| `hook` | Installs and manages Git `pre-commit` hooks to block secrets before they are committed. | [Execute hook](../cli/hook.md) |
| `init` | Initiates your local configuration (`.woof.toml`) and custom rulesets. | [Initialize Configuration (init)](../cli/init.md) |
| `check` | Designed for CI/CD. Analyzes full Git histories or specific commit ranges for leaked credentials. | [Execute check](../cli/check.md) |
| `patrol` | Execute a full perimeter sweep (`scan` + `audit`) | [Execute patrol](../cli/patrol.md) |
| `update` | Fetches the latest version of the tool. | [Execute update](../cli/update.md) |

???+ tip "Next Steps"
    Ready to see it in action? Head over to the **[Execute scan](../cli/scan.md)** to run your first security sweep.
