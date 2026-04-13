# Architecture

## Purpose

`ARCHITECTURE.md` details the architectural layout, modules, and paradigms driving `bridle-ctl`.

## Overview

- **SDK (`bridle-sdk`)**: Core models, encoding tools, DB models, and pipeline batch DB modules.
  - Supports dynamic database execution, dynamically connecting to PostgreSQL or SQLite via connection strings.
  - Runs diesel migrations dynamically per provider.
  - Provides thread-safe DB transactions and per-file locking (`file_lock.rs`).
- **CLI (`bridle-cli`)**: Front-end commands for DB ingestion, batch processing, and PR syncing.
  - Implements upstream PR syncing, incorporating smart fork checks (reusing forks across org/personal accounts) and dispatching upstream PRs.
  - Rate limiting protects upstream infrastructure (e.g., sending max 10 PRs per hour) preventing spam constraints.
- **Agent (`bridle-agent`)**: MCP protocol implementation providing AI Engineering features.
- **REST & RPC (`bridle-rest` / `bridle-rpc`)**: External communication and programmatic integration.
- **Frontend UI (`bridle-ui`)**: Angular 17+ standalone web application interfacing with the REST API. Provides visual dashboards for system health, org/repo management, and batch pipeline monitoring.

## FFI Integration

Compiled tools communicate through C ABIs (defined in `ffi.rs`) to mutate trees securely without unconstrained prompt generation.
