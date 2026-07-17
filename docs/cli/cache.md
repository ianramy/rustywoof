# Cache Management (cache)

To ensure high performance in CI/CD pipelines, Rustywoof caches threat intelligence data locally. Occasionally, you may need to clear this state to resolve stale metadata or to force a fresh synchronization with the upstream OSV database.

## Purge the Cache

```bash {.mac-terminal}
woof cache clean
```

## Understand Cache Operations

Rustywoof utilizes a versioned caching strategy to maintain compatibility with schema updates from the OSV database. When you run `woof cache clean`, the engine performs the following:

1. :material-folder-remove: **Sweep Strategy:** The tool iterates through all existing versions of the `woof_osv_cache_v*` directories located in your system's temporary storage.
2. :material-delete-sweep: **Purge Operation:** It aggressively removes these directories, ensuring that all cached API responses are fully discarded.
3. :material-refresh: **Fresh Synchronization:** Upon your next `woof audit` or `patrol` command, the engine will perform a cold-start query to fetch the most recent vulnerability intelligence.

!!! warning "Network Impact"
    `woof cache clean` will cause the next execution of `woof audit` to perform a full network request to the OSV API. Ensure you have an active internet connection after running this command to allow the engine to re-populate its threat intelligence store.