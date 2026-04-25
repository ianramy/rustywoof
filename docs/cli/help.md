# Execute Help (help)

The `woof help` command provides a rapid terminal reference for all available actions, arguments, and flags without requiring you to leave your IDE or open a browser.

## Access Built-in Documentation

```bash {.mac-terminal}
# (1) View top-level commands
woof help

# (2) View specific command documentation
woof help audit
```

1. :material-information-outline: Displays a summary of commands like `scan`, `audit`, `patrol`, and `hook`.
2. :material-magnify: Appending a command name outputs the specific parameter and flag table for that module.

!!! tip "Alias"
    You can achieve the exact same output by appending the `-h` or `--help` flag to any command (e.g., `woof scan --help`).