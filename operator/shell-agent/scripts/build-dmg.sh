#!/usr/bin/env bash
# build-dmg.sh — Build Vibecrafted.app (Release) + package as DMG
# Created by M&K (c)2026 VetCoders
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$REPO_ROOT/operator/shell-agent"

# Config
APP_NAME="Vibecrafted"
BUNDLE_ID="io.vetcoders.vibecrafted"
VERSION="0.1.0"
DMG_NAME="${APP_NAME}-${VERSION}-arm64"
BUILD_DIR="${PWD}/build"
APP_PATH="${BUILD_DIR}/${APP_NAME}.app"
DMG_PATH="${BUILD_DIR}/${DMG_NAME}.dmg"
SIGNING_IDENTITY="${SIGNING_IDENTITY:--}"

echo "=== Vibecrafted DMG builder ==="
echo "Version: ${VERSION}"
echo "Signing: ${SIGNING_IDENTITY}"

echo "=== [1/5] Rust release build ==="
cargo build -p vibecrafted-shell-ffi --release

echo "=== [2/5] Swift bindings ==="
cargo run -p uniffi-bindgen -- generate \
    --library ../../target/release/libvibecrafted_shell_ffi.dylib \
    --language swift \
    --out-dir app/Vibecrafted/Bridge/

echo "=== [3/5] Xcode build (Release) ==="
cd app && xcodegen generate 2>/dev/null && cd ..

rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}"

DERIVED_DATA="${BUILD_DIR}/DerivedData"

set -o pipefail
xcodebuild \
    -project app/Vibecrafted.xcodeproj \
    -scheme Vibecrafted \
    -configuration Release \
    -derivedDataPath "${DERIVED_DATA}" \
    CODE_SIGN_IDENTITY="${SIGNING_IDENTITY}" \
    CODE_SIGN_STYLE=Manual \
    PRODUCT_BUNDLE_IDENTIFIER="${BUNDLE_ID}" \
    build 2>&1 | tail -5

BUILT_APP=$(find "${DERIVED_DATA}" -name "${APP_NAME}.app" -type d | head -1)
if [ -z "$BUILT_APP" ]; then
    echo "ERROR: ${APP_NAME}.app not found in DerivedData"
    exit 1
fi

cp -R "${BUILT_APP}" "${APP_PATH}"
echo "App: ${APP_PATH}"

echo "=== [4/5] Code signing ==="
if [ "${SIGNING_IDENTITY}" = "-" ]; then
    echo "Ad-hoc signing..."
    codesign --force --deep --sign - "${APP_PATH}"
else
    echo "Signing with: ${SIGNING_IDENTITY}"
    codesign --force --deep --options runtime --sign "${SIGNING_IDENTITY}" "${APP_PATH}"
    codesign --verify --deep --strict "${APP_PATH}"
fi

echo "=== [5/5] Creating DMG ==="
rm -f "${DMG_PATH}"

DMG_STAGING="${BUILD_DIR}/dmg-staging"
rm -rf "${DMG_STAGING}"
mkdir -p "${DMG_STAGING}"
cp -R "${APP_PATH}" "${DMG_STAGING}/"
ln -s /Applications "${DMG_STAGING}/Applications"

hdiutil create \
    -volname "${APP_NAME}" \
    -srcfolder "${DMG_STAGING}" \
    -ov \
    -format UDZO \
    "${DMG_PATH}" 2>/dev/null

rm -rf "${DMG_STAGING}"

if [ "${SIGNING_IDENTITY}" = "-" ]; then
    codesign --force --sign - "${DMG_PATH}"
else
    codesign --force --sign "${SIGNING_IDENTITY}" "${DMG_PATH}"
fi

echo "=== Done ==="
echo "DMG: ${DMG_PATH}"
