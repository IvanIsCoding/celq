#!/usr/bin/env bash

# Script to download releases and generate install.sh with SHA256 checksums
# Usage: ./generate-install-script.sh <version>
# Example: ./generate-install-script.sh v0.2.0

set -euo pipefail

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 v0.2.0"
  exit 1
fi

REPO="IvanIsCoding/celq"
TEMP_DIR=$(mktemp -d)

cleanup() {
  rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

echo "Downloading releases for $VERSION..."

# Download the archives (excluding FreeBSD)
declare -A DOWNLOADS=(
  ["macos-aarch64"]="celq-macos-aarch64.tar.gz"
  ["macos-x86_64"]="celq-macos-x86_64.tar.gz"
  ["windows-x86_64"]="celq-windows-x86_64.zip"
  ["linux-x86_64-musl"]="celq-linux-x86_64-musl.tar.gz"
  ["linux-aarch64-musl"]="celq-linux-aarch64-musl.tar.gz"
  ["linux-x86_64-gnu"]="celq-linux-x86_64-gnu.tar.gz"
  ["linux-aarch64-gnu"]="celq-linux-aarch64-gnu.tar.gz"
)

declare -A CHECKSUMS

for key in "${!DOWNLOADS[@]}"; do
  filename="${DOWNLOADS[$key]}"
  echo "Downloading $filename..."
  gh release download "$VERSION" \
    --repo "$REPO" \
    --pattern "$filename" \
    --dir "$TEMP_DIR"
  
  # Calculate SHA256
  if command -v sha256sum > /dev/null 2>&1; then
    checksum=$(sha256sum "$TEMP_DIR/$filename" | cut -d' ' -f1)
  elif command -v shasum > /dev/null 2>&1; then
    checksum=$(shasum -a 256 "$TEMP_DIR/$filename" | cut -d' ' -f1)
  else
    echo "Error: need sha256sum or shasum"
    exit 1
  fi
  
  CHECKSUMS[$key]=$checksum
  echo "  SHA256: $checksum"
done

echo ""
echo "Generating install.sh from template..."

# Read template and substitute values
TEMPLATE_FILE="template_install.sh"
OUTPUT_FILE="install.sh"

if [ ! -f "$TEMPLATE_FILE" ]; then
  echo "Error: $TEMPLATE_FILE not found"
  exit 1
fi

# Create the output with all substitutions
sed -e "s/{{CELQ_VERSION}}/${VERSION}/g" \
    -e "s/{{CHECKSUM_MACOS_AARCH64}}/${CHECKSUMS[macos-aarch64]}/g" \
    -e "s/{{CHECKSUM_MACOS_X86_64}}/${CHECKSUMS[macos-x86_64]}/g" \
    -e "s/{{CHECKSUM_WINDOWS_X86_64}}/${CHECKSUMS[windows-x86_64]}/g" \
    -e "s/{{CHECKSUM_LINUX_X86_64_MUSL}}/${CHECKSUMS[linux-x86_64-musl]}/g" \
    -e "s/{{CHECKSUM_LINUX_AARCH64_MUSL}}/${CHECKSUMS[linux-aarch64-musl]}/g" \
    -e "s/{{CHECKSUM_LINUX_X86_64_GNU}}/${CHECKSUMS[linux-x86_64-gnu]}/g" \
    -e "s/{{CHECKSUM_LINUX_AARCH64_GNU}}/${CHECKSUMS[linux-aarch64-gnu]}/g" \
    "$TEMPLATE_FILE" > "$OUTPUT_FILE"

chmod +x "$OUTPUT_FILE"

echo "✅ Generated $OUTPUT_FILE"
echo ""
echo "Checksums:"
for key in "${!CHECKSUMS[@]}"; do
  echo "  $key: ${CHECKSUMS[$key]}"
done