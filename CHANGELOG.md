# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2025-12-26

### Breaking Changes
- **Default paths changed** from `~/.rmcp_servers/rmcp_mux/` to `~/.rmcp-servers/rust-mux/`.
- **Proxy command** changed from `rmcp_mux_proxy` to `rust-mux-proxy`.

### Added
- **Daemon Status Socket** - Query running daemon status via Unix socket.
- **Heartbeat System** - Configurable health checks for MCP servers.
  - `heartbeat_enabled` - Enable/disable per-server heartbeat
  - `heartbeat_interval_ms` - Check interval (default: 30s)
  - `heartbeat_timeout_ms` - Timeout before marking unhealthy
- **Tray Dashboard** - Multi-server status view in system tray.
- **Standalone Build** - Inlined common types, no workspace dependencies.

### Changed
- Default socket directory: `~/.rmcp-servers/rust-mux/sockets`.
- Default service name: `rust-mux` (hyphenated).
- Detection now matches both `rust-mux` and legacy `rmcp_mux` patterns.
- Updated to Rust Edition 2024 (stable).

### Fixed
- Consistent naming across paths, commands, and documentation.

## [0.3.4] - 2025-12-20

### Fixed
- Minor bug fixes and stability improvements.

## [0.3.0] - 2025-12-04

### Added
- **Library-first architecture** – rust-mux is now an embeddable Rust library, not just a CLI tool.
- `MuxConfig` builder for programmatic configuration:
  ```rust
  let config = MuxConfig::new("/tmp/mcp.sock", "npx")
      .with_args(vec!["@mcp/server-memory".into()])
      .with_max_clients(10);
  ```
- `run_mux_server(config)` – blocking entry point for single mux server.
- `spawn_mux_server(config)` – non-blocking spawn returning `MuxHandle` for lifecycle control.
- `MuxHandle` with `shutdown()`, `wait()`, `is_running()` methods.
- `run_mux_with_shutdown(params, token)` – external `CancellationToken` support for custom shutdown logic.
- `check_health(socket_path)` – simple health check function.
- `CliOptions` trait for generic CLI parameter handling.
- `docs/integration.md` – comprehensive library integration guide.
- Feature flags: `cli` (wizard, scan, binaries) and `tray` (system tray icon).

### Changed
- **Rebranded: `rmcp_mux` → `rust-mux`.** Crate name hyphenated on crates.io per convention; module path `rust_mux`. Binary `rmcp_mux_proxy` → `rust_mux_proxy`. All internal imports `use rmcp_mux::` → `use rust_mux::`. User-facing `RMCP_MUX_*` environment variables preserved for backward compatibility.
- **Moved to Loctree org:** `https://github.com/VetCoders/rust-mux` → `https://github.com/Loctree/rust-mux`.

### Added
- Package metadata: `description`, `repository`, `homepage`, `documentation`, `readme`, `keywords`, `categories`, `license = "MIT OR Apache-2.0"`, and `authors = ["Maciej Gad <void@div0.space>", "Monika Szymanska <hello@vetcoders.io>"]` in `Cargo.toml` for proper crates.io listing and discovery.

## 0.2.0 - 2025-11-24

### Added
- Optional tray icon (`--tray`) showing live server status, client and pending counts, and restart reasons. ([5eefde4](https://github.com/LibraxisAI/rust_mux/commit/5eefde4))
- Config file support (JSON/YAML/TOML) with auto-detection and CLI overrides. ([5eefde4](https://github.com/LibraxisAI/rust_mux/commit/5eefde4))
- `rust-mux-proxy` helper binary plus launchd template and installer tweaks for easier setup. ([04e5402](https://github.com/LibraxisAI/rust_mux/commit/04e5402))
- GitHub Actions CI workflow for formatting, linting, testing, and coverage, including an async proxy forwarding test. ([ad2b9aa](https://github.com/LibraxisAI/rust_mux/commit/ad2b9aa))
- Mux hooks, Semgrep rules, and expanded README documentation. ([e80083c](https://github.com/LibraxisAI/rust_mux/commit/e80083c))
- `health` subcommand to resolve config and assert socket reachability, plus unit tests for healthy/missing sockets.

### Changed
- Refactored mux state management and tray functionality into dedicated `state` and `tray` modules, with tray dependencies gated behind an optional `tray` feature; CI updated to run with `--no-default-features`. ([0d60764](https://github.com/LibraxisAI/rust_mux/commit/0d60764), [ad2b9aa](https://github.com/LibraxisAI/rust_mux/commit/ad2b9aa))

## 0.1.5
- Added JSON status snapshots (`--status-file` / `status_file`) including PID, queue depth, request limits, restart/backoff settings.
- Hardened runtime: lazy child start, request size guard, request timeouts, capped restart backoff, max restarts.
- Config/Wizard/Scan updated to surface new fields; defaults documented in README.
- Status writer task for tray/automation; MuxState now tracks queue depth and child PID.
- Tests cover initialize cache, resets, status snapshots, and proxy; CI runs fmt/clippy/tests/tarpaulin with `--no-default-features` (tray off in CI).
