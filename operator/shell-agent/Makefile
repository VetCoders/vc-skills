# shell-agent - UniFFI bridge + SwiftUI app
# Created by VetCoders (c)2026 LibraxisAI

.PHONY: bindings xcode app dmg dmg-signed

bindings:
	@echo "Building shell-agent-ffi (release)..."
	@cargo build -p vibecrafted-shell-ffi --release
	@echo "Generating Swift bindings..."
	@cargo run -p uniffi-bindgen -- generate --library ../target/release/libvibecrafted_shell_ffi.dylib --language swift --out-dir app/Vibecrafted/Bridge/
	@echo "Bindings ready: app/Vibecrafted/Bridge/"

xcode:
	@cd app && xcodegen generate
	@echo "Xcode project generated: app/Vibecrafted.xcodeproj"

app: bindings xcode
	@echo "Building Vibecrafted.app..."
	@rm -rf build/DerivedData build/Vibecrafted.app
	@set -o pipefail && xcodebuild -project app/Vibecrafted.xcodeproj -scheme Vibecrafted -configuration Debug -derivedDataPath build/DerivedData build 2>&1 | tail -20
	@BUILT_APP=$$(find build/DerivedData -name Vibecrafted.app -type d | head -1); \
		if [ -z "$$BUILT_APP" ]; then \
			echo "ERROR: Vibecrafted.app not found in build/DerivedData"; \
			exit 1; \
		fi; \
		cp -R "$$BUILT_APP" build/Vibecrafted.app
	@echo "App built: build/Vibecrafted.app"

dmg:
	# TODO: Call scripts/build-dmg.sh here for multi-binary bundle (T6 follow-up)
	@bash scripts/build-dmg.sh


dmg-signed:
	# TODO: Call scripts/build-dmg.sh here for multi-binary bundle (T6 follow-up)
	@SIGNING_IDENTITY="Developer ID Application: Maciej Gad (MW223P3NPX)" bash scripts/build-dmg.sh

