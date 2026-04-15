# The .woof.toml File

The `.woof.toml` file is the central nervous system for your project's Rustywoof instance. It controls memory constraints, directory exclusion, and algorithmic strictness.

Here is a complete example of a generated `.woof.toml` file, followed by a breakdown of its core components:

```toml
# .woof.toml
version = "1.0"

[engine]
# Files larger than this will be bypassed to prevent OOM errors (in bytes)
max_file_size = 10_485_760 # 10 MB

[scanner]
# Global threshold for the Shannon Entropy mathematical filter (0.0 to 8.0)
# Lower values increase strictness; higher values reduce false positives.
entropy_threshold = 3.5

# Directories and files to completely ignore during sweeps
ignore_paths = [
    "tests/fixtures/",
    "node_modules/",
    "target/",
    "dist/",
    "*.min.js"
]

[audit]
# Fail the audit if vulnerabilities are found with a CVSS score above this threshold
fail_on_cvss = 7.0
```

Core Sections
[engine]
This section governs how Rustywoof interacts with your host system's resources.

max_file_size: The strict cutoff for file analysis. Massive log files or compressed database dumps will be automatically ignored to guarantee memory safety on constrained CI runners.

[scanner]
This section controls the behavior of the secret detection sweep.

entropy_threshold: Determines how "random" a string needs to be before it is considered a legitimate cryptographic secret. If you are getting too many false positives on variable names, try slightly increasing this value.

ignore_paths: A list of glob patterns or exact paths. Rustywoof uses this list to aggressively prune the file tree before the Aho-Corasick automaton even initializes, saving valuable milliseconds.

[!NOTE]
Rustywoof automatically respects your .gitignore file. You only need to add paths to ignore_paths if they are checked into version control but should be excluded from security sweeps (like a folder of dummy credentials used strictly for unit testing).

[audit]
This section manages the behavior of the Supply Chain Watchdog.

fail_on_cvss: Allows you to set a severity floor. For example, setting this to 7.0 means woof audit will only fail your build for High or Critical vulnerabilities, while logging Low and Medium threats as warnings.
