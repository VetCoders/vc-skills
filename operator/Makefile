.PHONY: gates fmt fmt-check lint test check workspace-check clean help bindings xcode app dmg

gates: fmt-check lint test

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

lint:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

check: workspace-check

workspace-check:
	cargo check --workspace

clean:
	cargo clean

bindings:
	cargo build -p shell-agent-ffi --release
	cargo run -p uniffi-bindgen -- generate --library target/release/libshell_agent_ffi.dylib --language swift --out-dir shell-agent/app/Vibecrafted/Bridge/

xcode:
	cd shell-agent/app && xcodegen generate

app: bindings xcode
	xcodebuild -project shell-agent/app/Vibecrafted.xcodeproj -scheme Vibecrafted -configuration Debug -derivedDataPath shell-agent/app/build/DerivedData build

dmg:
	@$(MAKE) -C shell-agent dmg

dmg-signed:
	@$(MAKE) -C shell-agent dmg-signed

help:
	@printf '%s\n' 'targets: gates fmt fmt-check lint test check workspace-check clean bindings xcode app dmg'
