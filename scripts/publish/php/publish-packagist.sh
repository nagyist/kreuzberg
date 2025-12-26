#!/usr/bin/env bash
#
# Publish PHP package to Packagist
#
# Note: Packagist automatically updates packages via Git webhook when a new tag is pushed.
# This script is primarily for verification and to ensure the package update is complete.
#
# Arguments:
#   $1: Package version (required)
#
# Environment variables:
#   - DRY_RUN: If 'true', skip actual publishing (default: false)
#
# Usage:
#   ./publish-packagist.sh "1.0.0"
#

set -euo pipefail

if [[ $# -lt 1 ]]; then
	echo "Usage: $0 <version>" >&2
	exit 1
fi

VERSION="$1"
DRY_RUN="${DRY_RUN:-false}"
PACKAGE_NAME="kreuzberg/kreuzberg"

echo "::group::Publishing to Packagist"
echo "Package: ${PACKAGE_NAME}"
echo "Version: ${VERSION}"
echo "Dry run: ${DRY_RUN}"

# Packagist auto-updates via GitHub webhook, so we just verify
echo "::notice::Packagist updates automatically via GitHub webhook"
echo "::notice::Waiting for Packagist to detect the new tag..."

if [[ "$DRY_RUN" == "true" ]]; then
	echo "::notice::Dry run mode - skipping Packagist verification"
	echo "::endgroup::"
	exit 0
fi

# Wait a bit for the webhook to process
echo "Waiting 30 seconds for webhook processing..."
sleep 30

# Verify the version is available on Packagist
MAX_ATTEMPTS=12 # 12 attempts * 10 seconds = 2 minutes
ATTEMPT=1

while [ $ATTEMPT -le $MAX_ATTEMPTS ]; do
	echo "Checking Packagist (attempt ${ATTEMPT}/${MAX_ATTEMPTS})..."

	# Fetch package metadata
	RESPONSE=$(curl \
		--silent \
		--show-error \
		--retry 3 \
		--retry-delay 5 \
		"https://repo.packagist.org/p2/${PACKAGE_NAME}.json" 2>/dev/null || echo "{}")

	# Check if version exists
	if echo "$RESPONSE" | jq -e ".packages[\"${PACKAGE_NAME}\"] | any(.version == \"${VERSION}\")" >/dev/null 2>&1; then
		echo "::notice::✓ Package ${PACKAGE_NAME}:${VERSION} is now available on Packagist"
		echo "::notice::View at: https://packagist.org/packages/${PACKAGE_NAME}#${VERSION}"
		echo "::endgroup::"
		exit 0
	fi

	if [ $ATTEMPT -lt $MAX_ATTEMPTS ]; then
		echo "Version not found yet, waiting 10 seconds..."
		sleep 10
	fi

	ATTEMPT=$((ATTEMPT + 1))
done

# If we get here, the version wasn't found
echo "::warning::Package version not found on Packagist after ${MAX_ATTEMPTS} attempts"
echo "::warning::This may be a timing issue. Check Packagist manually:"
echo "::warning::  https://packagist.org/packages/${PACKAGE_NAME}"
echo "::warning::The package should appear once the GitHub webhook is processed."

echo "::endgroup::"
exit 0
