# Usage

`bridle-ctl` offers several subcommands to run agents, tools, databases, and upstream syncs.

## General Operations
```bash
# Start the agent daemon
bridle agent

# Start REST or RPC APIs
bridle rest
bridle rpc

# Start the Angular Web UI (from within bridle-ui/)
cd bridle-ui && npm start
```

## Batch Processing & Fixes
```bash
# Batch fix issues across multiple repositories
bridle batch-fix --org "my-org" --issue "Fix deprecated API" --tools "ffi_fixer" --db-url "postgres://user:pass@localhost/bridle"

# Run a YAML/TOML pipeline
bridle batch-run --config pipeline.yml
bridle batch-resume --job-id 12
bridle batch-status --job-id 12
```

## Upstream Synchronization (PRs & Forks)
When you have analyzed and prepared numerous Pull Requests locally, you can sync them to the upstream real repositories (e.g., GitHub) using the `sync-prs` command.

```bash
bridle sync-prs --org "my-org" --db-url "sqlite://bridle.db" --max-prs-per-hour 10
```
This command automatically:
1. **Checks for forks**: Looks for an existing fork in your personal account or organizations.
2. **Reuses/Creates**: Uses the existing fork or creates a new one.
3. **Pushes**: Pushes the branch to the remote fork.
4. **Sends PR**: Sends the Pull Request to the upstream organization.
5. **Rate Limits**: Enforces the global limit set by `--max-prs-per-hour` to prevent flooding the upstream provider.
