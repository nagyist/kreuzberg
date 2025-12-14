#!/usr/bin/env bash
set -euo pipefail

mode="${1:-check}"

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || dirname "$(dirname "$(dirname "$(readlink -f "$0")")")")"
cd "$repo_root"

if [ ! -d "infrastructure" ]; then
	echo "No infrastructure/ directory found, skipping tflint"
	exit 0
fi

if ! command -v tflint >/dev/null 2>&1; then
	echo "Error: tflint not found. Run 'task terraform:install' first." >&2
	exit 1
fi

tflint_config="$repo_root/.tflint.hcl"
if [ ! -f "$tflint_config" ]; then
	echo "Warning: .tflint.hcl not found, using default configuration"
	tflint_config=""
fi

case "$mode" in
fix | --fix)
	echo "Running tflint with auto-fix..."
	cd infrastructure
	if [ -n "$tflint_config" ]; then
		tflint --init --config="$tflint_config"
		tflint --fix --config="$tflint_config" --recursive
	else
		tflint --init
		tflint --fix --recursive
	fi
	echo "tflint auto-fix complete"
	;;
check | --check)
	echo "Running tflint in check mode..."
	cd infrastructure
	if [ -n "$tflint_config" ]; then
		tflint --init --config="$tflint_config"
		tflint --config="$tflint_config" --recursive
	else
		tflint --init
		tflint --recursive
	fi
	echo "tflint check passed"
	;;
*)
	echo "Usage: $0 [fix|check]" >&2
	exit 2
	;;
esac
