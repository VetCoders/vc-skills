# rust-mux – AI-facing Overview

> **Version:** 0.3.0
> **Last updated:** 2025-12-04

This document provides a concise technical overview for AI agents working with the rust-mux codebase.

## Purpose

**Library-first MCP multiplexer** – share a single MCP server process across many hosts via Unix socket.

Two usage modes:
1. **As a library** – embed in Rust applications, run multiple MCP services in one process
2. **As a CLI** – standalone daemon with wizard, scan, and rewire commands

Core features:
- JSON-RPC ID rewriting per client
- `initialize` request caching and fan-out
- Request limits, timeouts, and size guards
- Child process restart with exponential backoff
- Status snapshots for UI/automation

## Quick Start

### Library Usage (Recommended)

```rust
use rust_mux::{MuxConfig, run_mux_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MuxConfig::new("/tmp/mcp.sock", "npx")
        .with_args(vec!["-y".into(), "@mcp/server-memory".into()])
        .with_max_clients(10);
    run_mux_server(config).await
}
```

### CLI Usage

```bash
# Build
cargo build --release

# Run mux daemon
./target/release/rust-mux \
  --socket ~/.rmcp-servers/rust-mux/sockets/memory.sock \
  --cmd npx -- @modelcontextprotocol/server-memory \
  --max-active-clients 5 \
  --status-file ~/.rmcp-servers/rust-mux/status.json

# Host side: use bundled proxy
rust-mux-proxy --socket ~/.rmcp-servers/rust-mux/sockets/memory.sock
```

## Project Structure (v0.3.0)

```
src/
├── lib.rs               # Library entry point, public API
├── config.rs            # Config, ServerConfig, ResolvedParams, CliOptions trait
├── state.rs             # MuxState, StatusSnapshot, error_response, set_id
├── scan.rs              # Host discovery, rewiring (feature: cli)
├── tray.rs              # Tray icon (feature: tray)
├── bin/
│   ├── rust-mux.rs      # CLI binary (feature: cli)
│   └── rust-mux-proxy.rs    # STDIO↔socket proxy (feature: cli)
├── runtime/             # Mux daemon core
│   ├── mod.rs           # run_mux, run_mux_internal, health_check
│   ├── types.rs         # ServerEvent, MAX_QUEUE, MAX_PENDING
│   ├── client.rs        # handle_client, handle_client_message
│   ├── server.rs        # server_manager, handle_server_events
│   ├── proxy.rs         # run_proxy (STDIO)
│   ├── status.rs        # write_status_file, spawn_status_writer
│   └── tests.rs         # All runtime tests
└── wizard/              # Three-step TUI wizard (feature: cli)
    ├── mod.rs           # run_wizard, run_tui
    ├── types.rs         # WizardStep, ServiceEntry, ClientEntry, FormState
    ├── services.rs      # load_all_services, detect_running_mcp_servers
    ├── clients.rs       # detect_clients
    ├── ui.rs            # draw_ui, draw_service_list, draw_client_list
    ├── keys.rs          # handle_key, sync_form_to_service
    └── persist.rs       # persist_all, rewire_selected_clients
```

## Library API

### Core Types

| Type | Description |
|------|-------------|
| `MuxConfig` | Builder for programmatic configuration |
| `MuxHandle` | Lifecycle control for spawned servers |
| `ResolvedParams` | Merged CLI + config parameters |
| `CliOptions` | Trait for generic CLI parameter handling |

### Entry Points

```rust
// Blocking - runs until Ctrl+C
run_mux_server(config: MuxConfig) -> Result<()>

// Non-blocking - returns handle for control
spawn_mux_server(config: MuxConfig) -> Result<MuxHandle>

// External shutdown control
run_mux_with_shutdown(params: ResolvedParams, token: CancellationToken) -> Result<()>

// Health check
check_health(socket: impl AsRef<Path>) -> Result<()>
```

### MuxConfig Builder

```rust
MuxConfig::new(socket, cmd)
    .with_args(vec![...])           // Command arguments
    .with_max_clients(10)           // Max concurrent clients
    .with_service_name("my-svc")    // For logging/status
    .with_request_timeout(Duration::from_secs(60))
    .with_lazy_start(true)          // Spawn on first request
    .with_status_file("/path")      // JSON snapshots
```

### MuxHandle Methods

| Method | Description |
|--------|-------------|
| `shutdown()` | Request graceful shutdown (non-blocking) |
| `wait().await` | Wait for server to terminate |
| `is_running()` | Check if server is still active |

## CLI Subcommands

| Command | Purpose |
|---------|---------|
| (default) | Run mux daemon |
| `wizard` | Three-step TUI: servers → clients → save |
| `scan` | Discover hosts, generate manifest/snippets |
| `rewire` | Update host config to use proxy |
| `status` | Check if host is rewired |
| `health` | Verify socket reachability |
| `proxy` | STDIO↔socket proxy |

## Config (JSON/YAML/TOML)

Default path: `~/.codex/mcp.json` (override `--config`, pick `--service` key under `servers.<name>`).

**Fields per service:**
- `socket`, `cmd`, `args` – required
- `max_active_clients` – default 5
- `lazy_start` – default false
- `max_request_bytes` – default 1_048_576
- `request_timeout_ms` – default 30_000
- `restart_backoff_ms` – default 1_000
- `restart_backoff_max_ms` – default 30_000
- `max_restarts` – default 5 (0 = unlimited)
- `tray`, `service_name`, `log_level`
- `status_file` – JSON snapshots for UI/automation

## Three-Step Wizard

```bash
rust-mux wizard --config ~/.codex/mcp-mux.toml
```

1. **Server Detection** – scans `ps` for MCP processes, loads config, toggles with `Space`
2. **Client Detection** – finds Codex/Cursor/VSCode/Claude/JetBrains configs, shows rewire status
3. **Confirmation** – save options: Save All, Mux Only, Clipboard, Back, Exit

Navigation: `n` next, `p` previous, `Space` toggle, `Tab` switch panel, `q` quit.

## Status Snapshots

Written atomically to `status_file` on every state change:
```json
{
  "service_name": "memory",
  "server_status": "Running",
  "restarts": 0,
  "connected_clients": 2,
  "active_clients": 1,
  "pending_requests": 0,
  "queue_depth": 0,
  "child_pid": 12345,
  "cached_initialize": true
}
```

## Testing

```bash
# Full suite (40 tests)
cargo test

# Without tray feature (CI/headless)
cargo test --no-default-features

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Coverage
cargo tarpaulin --all-targets --no-default-features --out Lcov
```

## Key Symbols for Navigation

| Symbol | Location | Purpose |
|--------|----------|---------|
| `MuxConfig` | lib.rs | Builder for programmatic configuration |
| `MuxHandle` | lib.rs | Lifecycle control (shutdown, wait, is_running) |
| `run_mux_server` | lib.rs | Blocking server entry point |
| `spawn_mux_server` | lib.rs | Non-blocking spawn returning MuxHandle |
| `run_mux_with_shutdown` | lib.rs | External CancellationToken support |
| `check_health` | lib.rs | Socket health check |
| `CliOptions` | config.rs | Trait for generic CLI parameter handling |
| `ResolvedParams` | config.rs | Merged CLI + config parameters |
| `MuxState` | state.rs | Runtime state (clients, pending, cache) |
| `StatusSnapshot` | state.rs | JSON status output |
| `run_mux` | runtime/mod.rs | Main mux loop (with internal shutdown) |
| `run_mux_internal` | runtime/mod.rs | Main mux loop (external shutdown) |
| `server_manager` | runtime/server.rs | Child process lifecycle |
| `handle_client` | runtime/client.rs | Client connection handler |
| `run_wizard` | wizard/mod.rs | TUI entry point (feature: cli) |
| `WizardStep` | wizard/types.rs | Step enum (Server/Client/Confirmation) |
| `discover_hosts` | scan.rs | Find host config files (feature: cli) |

## Notes for AI Agents

1. **Library-first architecture:** Use `MuxConfig` + `spawn_mux_server` for embedding. CLI is feature-gated.

2. **Feature gating:**
   - `cli` feature: wizard, scan, binaries (clap, ratatui, crossterm)
   - `tray` feature: system tray icon (tray-icon, image)
   - Build with `--no-default-features` for library-only.

3. **Naming convention:**
   - Package name: `rust-mux` (crates.io, Cargo.toml)
   - Library name: `rust_mux` (Rust identifier, `use rust_mux::*`)
   - Binary names: `rust-mux`, `rust-mux-proxy`
- Share a single MCP server process (e.g., `npx @modelcontextprotocol/server-memory`) across many MCP hosts via a Unix socket.
- Rewrite JSON-RPC IDs per client, cache `initialize`, enforce request limits/timeouts, restart child with backoff, and expose status for UI/automation.
## Quick start
# Run mux daemon
./target/release/rust-mux \
  --socket ~/.rmcp-servers/rust-mux/sockets/memory.sock \
  --status-file ~/.rmcp-servers/rust-mux/status.json

# Host side: use bundled proxy
rust-mux-proxy --socket ~/.rmcp-servers/rust-mux/sockets/memory.sock

4. **Single child model:** One MCP server per socket. Multiple services = multiple MuxConfig instances.

5. **Initialize caching:** First `initialize` is cached in `MuxState.cached_initialize`. Later clients get cached response via `init_waiting`.

6. **Error handling:** Use `anyhow::Result` and `.with_context()` for all fallible operations.

## Status snapshots
- Written atomically to `status_file` on every state change.
- Contains: service_name, server_status, restarts, connected/active clients, pending count, queue_depth, child_pid, max_request_bytes, restart backoff settings, last_reset, initialize cache flag.

## Tooling / commands
- `scan`: discover host configs (Codex/Cursor/Claude/JetBrains), build mux manifest/snippets.
- `rewire`: apply proxy command into host configs (creates `.bak`; `--dry-run` to preview).
- `status`: check host configs point to `rust-mux-proxy`.
- `wizard` (ratatui): guided editor for mux config and host rewiring (writes backups; `--dry-run` supported).

## Testing / CI
- Local: `cargo fmt`, `cargo clippy --all-targets --no-default-features -- -D warnings`, `cargo test --no-default-features`.
- Coverage: `cargo tarpaulin --all-targets --no-default-features --out Lcov`.
- CI workflow: `.github/workflows/ci.yml` (fmt/clippy/test/tarpaulin, tray feature off).

## Notes for agents
- Comments/docs in English only.
- Tray feature is feature-gated; CI builds with `--no-default-features` to avoid GUI deps.
- `.ai-agents/**` is scratch space (do not commit). `AGENTS.md` is deprecated/cringe; ignore.
- Prefer `rust-mux-proxy` over `socat` for host STDIO integration.

