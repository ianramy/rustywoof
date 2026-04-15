# Configuration

Rustywoof is designed to be devastatingly effective out-of-the-box with zero configuration. However, enterprise environments often require granular control over performance boundaries, ignore lists, and domain-specific security rules.

All configuration is managed through a single file located at the root of your repository: `.woof.toml`.

### Getting Started

To generate a default configuration file with best practices pre-applied, run:

```bash
woof init
```

This command will create a .woof.toml file in your current working directory. You should commit this file to version control so your entire team and CI/CD pipeline share the same security baselines.

Explore Configuration Topics
The .woof.toml File: A complete breakdown of global settings, ignore paths, and performance tuning.

Custom Rules: How to define domain-specific regex patterns and leverage the Aho-Corasick engine for proprietary secrets.
