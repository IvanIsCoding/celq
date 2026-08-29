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

# Backup the original file first
BACKUP_FILE="$NIX_FILE.backup"
cp "$NIX_FILE" "$BACKUP_FILE"

# Function to restore original file on exit
cleanup() {
    if [ -f "$BACKUP_FILE" ]; then
        cp "$BACKUP_FILE" "$NIX_FILE"
        rm -f "$BACKUP_FILE"
        rm -f "$NIX_FILE.bak" 2>/dev/null || true
        echo
        echo "✓ Original file restored"
    fi
}
trap cleanup EXIT

# Step 1: Update version
echo "Step 1: Updating version to $NEW_VERSION"
sed -i.bak "s/version = \"[^\"]*\";/version = \"$NEW_VERSION\";/" "$NIX_FILE"
rm -f "$NIX_FILE.bak"
echo

# Step 2: Calculate fetchCrate hash
echo "Step 2: Calculating fetchCrate hash..."
echo "(This may take a moment...)"

# Use Nix's fake SHA-256 so the fetcher downloads through the normal verified
# path and reports the source hash for the requested crate version.
sed -i.bak 's|sha256 = "sha256-[^"]*";|sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";|' "$NIX_FILE"
rm -f "$NIX_FILE.bak"

set +e
BUILD_OUTPUT=$(nix build . 2>&1)
set -e

# Extract hash from build output
FETCH_HASH=$(echo "$BUILD_OUTPUT" | grep -oP "got:\s+sha256-\K[A-Za-z0-9+/=]+" | head -1 || true)

if [ -z "$FETCH_HASH" ]; then
    echo "❌ Failed to extract fetchCrate hash"
    echo "Build output:"
    echo "$BUILD_OUTPUT"
    exit 1
fi

FETCH_HASH_FORMATTED="sha256-$FETCH_HASH"
echo "✓ fetchCrate hash: $FETCH_HASH_FORMATTED"
echo

# Update the file with the correct fetchCrate hash
sed -i.bak "s|sha256 = \"sha256-[^\"]*\";|sha256 = \"$FETCH_HASH_FORMATTED\";|" "$NIX_FILE"
rm -f "$NIX_FILE.bak"

# Step 3: Calculate cargoHash
echo "Step 3: Calculating cargoHash..."
echo "(This will trigger a build error to extract the correct hash...)"

# Set dummy hash to force error
sed -i.bak 's/cargoHash = "sha256-[^"]*";/cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";/' "$NIX_FILE"
rm -f "$NIX_FILE.bak"

set +e
BUILD_OUTPUT=$(nix build . 2>&1)
set -e

# Extract cargoHash from error message
CARGO_HASH=$(echo "$BUILD_OUTPUT" | grep -oP "got:\s+sha256-\K[A-Za-z0-9+/=]+" | tail -1 || true)

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
