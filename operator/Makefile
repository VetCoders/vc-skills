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
	@echo "TODO(T4): generate UniFFI bindings for the macOS shell agent."

xcode:
	@echo "TODO(T4): generate or open the Xcode project."

app:
	@echo "TODO(T4): build the macOS app target."

dmg:
	@echo "TODO(T4): package the macOS app as a DMG."

help:
	@printf '%s\n' 'targets: gates fmt fmt-check lint test check workspace-check clean bindings xcode app dmg'
