#!/usr/bin/env bash
set -euo pipefail

# Script to calculate celq nixpkg hashes for a new version
# Usage: .github/scripts/nix_hashes.sh <new_version>

NEW_VERSION="${1:-}"

if [ -z "$NEW_VERSION" ]; then
    echo "Error: Please provide a version number"
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.3.0"
    exit 1
fi

NIX_FILE="nix/celq.nix"

if [ ! -f "$NIX_FILE" ]; then
    echo "Error: $NIX_FILE not found"
    exit 1
fi

echo "================================================"
echo "Calculating hashes for celq v$NEW_VERSION"
echo "================================================"
echo

# Create temporary file
TMP_FILE="$NIX_FILE.tmp"
cp "$NIX_FILE" "$TMP_FILE"

# Update version in temporary file
sed -i.bak "s/version = \"[^\"]*\";/version = \"$NEW_VERSION\";/" "$TMP_FILE"
echo "Step 1: Updated version to $NEW_VERSION in temporary file"
echo

# Replace main file temporarily
BACKUP_FILE="$NIX_FILE.backup"
mv "$NIX_FILE" "$BACKUP_FILE"
mv "$TMP_FILE" "$NIX_FILE"

# Function to restore original file on exit
cleanup() {
    if [ -f "$BACKUP_FILE" ]; then
        mv "$BACKUP_FILE" "$NIX_FILE"
        rm -f "$NIX_FILE.bak" "$TMP_FILE.bak" 2>/dev/null || true
        echo
        echo "✓ Original file restored"
    fi
}
trap cleanup EXIT

# Step 2: Calculate fetchCrate hash
echo "Step 2: Calculating fetchCrate hash..."
echo "(This may take a moment...)"

set +e
BUILD_OUTPUT=$(nix build .#celq 2>&1)
FETCH_HASH=$(echo "$BUILD_OUTPUT" | grep -oP "got:\s+sha256-\K[A-Za-z0-9+/=]+" | head -1)
set -e

if [ -z "$FETCH_HASH" ]; then
    # If the build didn't fail, extract current hash
    FETCH_HASH=$(grep -oP 'sha256 = "sha256-\K[^"]+' "$NIX_FILE" | head -1)
fi

if [ -z "$FETCH_HASH" ]; then
    echo "❌ Failed to extract fetchCrate hash"
    echo "Build output:"
    echo "$BUILD_OUTPUT"
    exit 1
fi

FETCH_HASH_FORMATTED="sha256-$FETCH_HASH"
echo "✓ fetchCrate hash: $FETCH_HASH_FORMATTED"
echo

# Update the temporary file with the correct fetchCrate hash
sed -i.bak "s|sha256 = \"sha256-[^\"]*\";|sha256 = \"$FETCH_HASH_FORMATTED\";|" "$NIX_FILE"

# Step 3: Calculate cargoHash
echo "Step 3: Calculating cargoHash..."
echo "(This will trigger a build error to extract the correct hash...)"

# Set dummy hash
sed -i.bak 's/cargoHash = "sha256-[^"]*";/cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";/' "$NIX_FILE"

set +e
BUILD_OUTPUT=$(nix build .#celq 2>&1)
CARGO_HASH=$(echo "$BUILD_OUTPUT" | grep -oP "got:\s+sha256-\K[A-Za-z0-9+/=]+" | tail -1)
set -e

if [ -z "$CARGO_HASH" ]; then
    echo "❌ Failed to extract cargoHash"
    echo "Build output:"
    echo "$BUILD_OUTPUT"
    exit 1
fi

CARGO_HASH_FORMATTED="sha256-$CARGO_HASH"
echo "✓ cargoHash: $CARGO_HASH_FORMATTED"
echo

# Output results
echo "================================================"
echo "Hash Calculation Results for celq v$NEW_VERSION"
echo "================================================"
echo
echo "Update the following values in nix/celq.nix:"
echo
echo "  version = \"$NEW_VERSION\";"
echo
echo "  sha256 = \"$FETCH_HASH_FORMATTED\";"
echo
echo "  cargoHash = \"$CARGO_HASH_FORMATTED\";"
echo
echo "================================================"

# Export for GitHub Actions
if [ -n "${GITHUB_OUTPUT:-}" ]; then
    echo "FETCH_HASH=$FETCH_HASH_FORMATTED" >> "$GITHUB_OUTPUT"
    echo "CARGO_HASH=$CARGO_HASH_FORMATTED" >> "$GITHUB_OUTPUT"
fi