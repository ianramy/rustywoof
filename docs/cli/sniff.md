# Dependency Sniffing (sniff)

The dependency landscape in modern projects can be complex, with deeply nested transitive requirements. The `woof sniff` command provides deep visibility into your dependency graph, allowing you to trace exactly how a specific package entered your environment.

## Execute the Sniff

```bash {.mac-terminal}
woof sniff <package-name>
```

## Analyze the Dependency Trace

When you target a specific package, `woof sniff` constructs a tree view that reveals the complete lineage from your workspace root to the requested node.

1. :material-graph: **Workspace Roots:** Identifies all entry points in your project that pull in the dependency.
2. :material-source-branch: **Transitive Resolution:** Maps out the entire parent chain, showing exactly which intermediate libraries are introducing the package.
3. :material-package: **Metadata Correlation:** Displays the name, version, and ecosystem for every node in the tree, allowing for immediate identification of version mismatches.

!!! note "Why Sniff?"
    If `woof audit` flags a vulnerability in a transitive dependency that you do not directly list in your `Cargo.toml` or `package.json`, `woof sniff` is the primary tool to discover which of your direct dependencies is responsible for pulling that vulnerable version in.
