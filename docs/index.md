# Bridle Documentation

Welcome to the Bridle documentation portal.

`bridle-ctl` is an AI-native codebase orchestrator and embedded Git Forge backend. This portal unifies our standard written documentation with the auto-generated code references for both our Rust backend and Angular frontend.

## 📚 API References

The following references are auto-generated from the source code during the `make docs` build process:

*   🦀 **[Rust SDK & CLI API Reference](/rust/bridle_cli/index.html)** (Generated via `cargo doc`)
*   🅰️ **[Angular UI Reference](/angular/index.html)** (Generated via `compodoc`)

---

## Getting Started

To build these documents locally:

1. Install system dependencies: `make install_base`
2. Install project dependencies: `make install_deps`
3. Build the full unified site: `make docs`

The completed site will be available in the `site/` directory. You can preview it by running `python3 -m mkdocs serve` and then navigating to `http://localhost:8000/rust/bridle_cli/index.html` or the Angular counterpart.
