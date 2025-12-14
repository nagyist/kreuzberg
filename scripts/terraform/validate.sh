#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || dirname "$(dirname "$(dirname "$(readlink -f "$0")")")")"
cd "$repo_root"

if [ ! -d "infrastructure" ]; then
	echo "No infrastructure/ directory found, skipping validation"
	exit 0
fi

if ! command -v tofu >/dev/null 2>&1; then
	echo "Error: OpenTofu not found. Run 'task terraform:install' first." >&2
	exit 1
fi

echo "Validating Terraform configuration..."
cd infrastructure

if [ ! -f ".terraform.lock.hcl" ]; then
	echo "Initializing Terraform..."
	tofu init -backend=false
fi

tofu validate

echo "Terraform validation passed"
