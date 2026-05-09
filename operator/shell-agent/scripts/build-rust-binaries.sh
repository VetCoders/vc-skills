#!/usr/bin/env bash
# Helper script to build rust binaries.
# Called by the Xcode Run Script build phase or independently.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "Building Rust binaries (Helper)..."
cd "$REPO_ROOT"
cargo build -p rust-mux --release --bin rust-mux
cargo build -p tray-agent --release --bin vc-mux-tray
cargo build -p vibecrafted-operator --release --bin vc-operator
cargo build -p shell-agent-ffi --release
