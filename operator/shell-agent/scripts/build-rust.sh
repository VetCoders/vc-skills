#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$REPO_ROOT/operator"

echo "=== Building vibecrafted-shell-ffi (release) ==="
cargo build -p vibecrafted-shell-ffi --release

echo "=== Generating Swift bindings ==="
cargo run -p uniffi-bindgen -- generate \
    --library ../target/release/libvibecrafted_shell_ffi.dylib \
    --language swift \
    --out-dir shell-agent/app/Vibecrafted/Bridge/

echo "=== Done ==="
