#!/usr/bin/env bash
#
# Run PHP tests
# Used by: ci-php.yaml - Run PHP tests step
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# scripts/ci/php lives three levels below repo root
REPO_ROOT="${REPO_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"

source "$REPO_ROOT/scripts/lib/common.sh"
source "$REPO_ROOT/scripts/lib/library-paths.sh"

validate_repo_root "$REPO_ROOT" || exit 1
setup_rust_ffi_paths "$REPO_ROOT"

echo "=== Running PHP tests ==="
cd "$REPO_ROOT/packages/php"

echo ""
echo "=== Pre-test environment ==="
echo "PHP version: $(php --version)"
echo "Working directory: $(pwd)"
echo ""

echo "=== Library search paths ==="
echo "LD_LIBRARY_PATH: ${LD_LIBRARY_PATH:-<not set>}"
echo "DYLD_LIBRARY_PATH: ${DYLD_LIBRARY_PATH:-<not set>}"
echo ""

echo "=== Checking for PHP extension ==="
if [ -f "ext/libkreuzberg_php.so" ] || [ -f "ext/libkreuzberg_php.dylib" ] || [ -f "ext/kreuzberg_php.dll" ]; then
	echo "PHP extension found"
	ls -lh ext/libkreuzberg_php.* 2>/dev/null || ls -lh ext/kreuzberg_php.* 2>/dev/null || true
else
	echo "WARNING: PHP extension not found in ext/"
fi
echo ""

echo "=== Running PHPUnit tests ==="
composer exec -- phpunit --verbose --testdox || {
	echo ""
	echo "ERROR: PHPUnit tests failed"
	exit 1
}

echo ""
echo "=== Tests complete ==="
