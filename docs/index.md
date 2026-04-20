# Bridle Documentation

Welcome to the Bridle documentation portal.

`bridle-ctl` is an AI-native codebase orchestrator and embedded Git Forge backend. This portal unifies our standard written documentation with the auto-generated code references for both our Rust backend and Angular frontend.

---

## 🧭 Navigation

- **[README](../README.md)**: High-level overview of the project.
- **[Architecture](../ARCHITECTURE.md)**: Deep dive into the component design and FFI integration.
- **[Usage Guide](../USAGE.md)**: CLI commands, APIs, and batch pipeline operations.
- **[Adding New Tools](../ADD_NEW_TOOLS.md)**: How to write, compile, and bind new FFI tools.
- **[Agent Guidelines](../AGENT_GUIDELINES.md)**: Operational rules for the AI context.
- **[Web UI](../bridle-ui/README.md)**: Information about the Angular dashboard.

---

## 📚 Auto-Generated API References

The following references are auto-generated from the source code during the `make docs` build process. They provide detailed documentation on every struct, function, and component in the system.

- 🦀 **[Rust SDK & CLI API Reference](/rust/bridle_cli/index.html)** (Generated via `cargo doc`)
- 🅰️ **[Angular UI Reference](/angular/index.html)** (Generated via `compodoc`)

---

## 🛠️ Building the Docs Locally

To build this unified documentation site locally:

1. Install system dependencies (MkDocs, Cargo, Compodoc):
   ```bash
   make install_base
   ```
2. Install project dependencies:
   ```bash
   make install_deps
   ```
3. Build the full unified site:
   ```bash
   make docs
   ```

The completed static site will be available in the `site/` directory. You can preview it by running `python3 -m mkdocs serve` and then navigating to `http://localhost:8000/`.
