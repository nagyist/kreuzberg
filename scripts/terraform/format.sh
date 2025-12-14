#!/usr/bin/env bash
set -euo pipefail

mode="${1:-fix}"

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || dirname "$(dirname "$(dirname "$(readlink -f "$0")")")")"
cd "$repo_root"

files="$(git ls-files 'infrastructure/**/*.tf' '*.tf' 2>/dev/null | grep -v 'vendor/\|node_modules/\|target/' || true)"

if [ -z "$files" ]; then
	echo "No Terraform files found"
	exit 0
fi

case "$mode" in
--check | check)
	echo "Checking Terraform formatting..."
	# shellcheck disable=SC2086
	tofu fmt -check -recursive infrastructure/ || {
		echo "Error: Terraform files need formatting. Run 'task terraform:fmt' to fix." >&2
		exit 1
	}
	echo "Terraform formatting check passed"
	;;
fix | --fix)
	echo "Formatting Terraform files..."
	# shellcheck disable=SC2086
	tofu fmt -recursive infrastructure/
	echo "Terraform formatting complete"
	;;
*)
	echo "Usage: $0 [fix|check|--check|--fix]" >&2
	exit 2
	;;
esac
