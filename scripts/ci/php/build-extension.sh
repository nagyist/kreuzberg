#!/usr/bin/env bash
#
# Build PHP extension
# Used by: ci-php.yaml - Build PHP extension step
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# scripts/ci/php lives three levels below repo root
REPO_ROOT="${REPO_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"

source "$REPO_ROOT/scripts/lib/common.sh"
source "$REPO_ROOT/scripts/lib/library-paths.sh"

validate_repo_root "$REPO_ROOT" || exit 1
setup_rust_ffi_paths "$REPO_ROOT"

echo "=== Building PHP extension ==="
cd "$REPO_ROOT"

# Ensure target/release directory exists
mkdir -p target/release

# Build the Rust FFI library if not already built
if [ ! -f "target/release/libkreuzberg_php.so" ] && [ ! -f "target/release/libkreuzberg_php.dylib" ] && [ ! -f "target/release/kreuzberg_php.dll" ]; then
	echo "Building Rust FFI library..."
	cargo build --release -p kreuzberg-php
else
	echo "Rust FFI library already built"
fi

# Copy the built library to the PHP extension directory
echo "Copying built library to packages/php/ext/"
mkdir -p "$REPO_ROOT/packages/php/ext"

if [ -f "target/release/libkreuzberg_php.so" ]; then
	cp -v target/release/libkreuzberg_php.so packages/php/ext/
elif [ -f "target/release/libkreuzberg_php.dylib" ]; then
	cp -v target/release/libkreuzberg_php.dylib packages/php/ext/
elif [ -f "target/release/kreuzberg_php.dll" ]; then
	cp -v target/release/kreuzberg_php.dll packages/php/ext/
else
	echo "ERROR: Could not find built library"
	exit 1
fi

echo ""
echo "=== Build complete ==="
echo "Extension library available at: packages/php/ext/"
ls -lh packages/php/ext/
