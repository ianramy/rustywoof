# Initialize Configuration (init)

The `woof init` command generates a standardized `woof.toml` configuration file in the root of your current working directory. 

## Execute Initialization

```bash {.mac-terminal}
woof init
```

## Understand the Configuration

Why initialize a config file? While Rustywoof is designed to work out-of-the-box with sane defaults, enterprise environments often require specific tuning. By initializing a configuration file, you establish a baseline that allows you to:

* **Define custom regex rules** for internal, proprietary token formats.
* **Whitelist false positives** via file paths or specific Git commits.
* **Override memory constraints** like the maximum file size limit.

???+ info "Zero-Config Fallback"
    If no `woof.toml` is present, Rustywoof simply uses its compiled-in default engine constraints and baseline threat signatures.
