# Configure the Tool

Rustywoof is designed to be devastatingly effective out-of-the-box with zero configuration. However, enterprise environments often require granular control over performance boundaries, ignore lists, and domain-specific security rules.

All configuration is managed through a single file located at the root of your repository: `.woof.toml`.

## Initialize Configuration

To generate a default configuration file with best practices pre-applied, execute the initialization command:

```bash {.mac-terminal}
woof init
```

!!! tip "Version Control Integration"
    You should commit the generated `.woof.toml` file to version control. This ensures your entire team, local Git hooks, and CI/CD pipelines share the exact same security baselines and ignore rules.

## Explore Configuration Domains

<div class="grid cards" markdown>

-   :material-file-cog:{ .lg .middle } __Modify .woof.toml__

    ---

    Learn how to adjust global settings, memory constraints, CVSS thresholds, and ignore paths.

    [:octicons-arrow-right-24: Modify woof.toml](woof-toml.md)

-   :material-regex:{ .lg .middle } __Design Custom Rules__

    ---

    Define domain-specific regex patterns and leverage the Aho-Corasick engine to hunt for proprietary secrets.

    [:octicons-arrow-right-24: Design Custom Rules](custom-rules.md)

</div>
