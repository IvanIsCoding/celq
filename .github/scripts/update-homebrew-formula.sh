#!/usr/bin/env bash
set -euo pipefail

# Check required arguments
if [ $# -ne 4 ]; then
  echo "Usage: $0 <version> <repo> <template_path> <output_path>"
  echo "Example: $0 v1.0.0 IvanIsCoding/celq brew/celq.rb Formula/celq.rb"
  exit 1
fi

VERSION="$1"
REPO="$2"
TEMPLATE_PATH="$3"
OUTPUT_PATH="$4"

VERSION_WITHOUT_V="${VERSION#v}"
TEMP_DIR=$(mktemp -d)

echo "Updating Homebrew formula to version: $VERSION_WITHOUT_V"
echo "Temporary directory: $TEMP_DIR"

# Define the archives we need to download
declare -A DOWNLOADS=(
  ["macos-aarch64"]="celq-macos-aarch64.tar.gz"
  ["macos-x86_64"]="celq-macos-x86_64.tar.gz"
  ["linux-x86_64-gnu"]="celq-linux-x86_64-gnu.tar.gz"
  ["linux-aarch64-gnu"]="celq-linux-aarch64-gnu.tar.gz"
)

declare -A CHECKSUMS

# Download archives and calculate checksums
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

# Generate formula from template
echo "Generating formula from template..."
sed -e "s/{{CELQ_VERSION}}/${VERSION_WITHOUT_V}/g" \
    -e "s/{{CELQ_SHA256_MACOS_ARM64}}/${CHECKSUMS[macos-aarch64]}/g" \
    -e "s/{{CELQ_SHA256_MACOS_X86_64}}/${CHECKSUMS[macos-x86_64]}/g" \
    -e "s/{{CELQ_SHA256_LINUX_ARM64}}/${CHECKSUMS[linux-aarch64-gnu]}/g" \
    -e "s/{{CELQ_SHA256_LINUX_X86_64}}/${CHECKSUMS[linux-x86_64-gnu]}/g" \
    "$TEMPLATE_PATH" > "$OUTPUT_PATH"

# Cleanup
rm -rf "$TEMP_DIR"

echo "✅ Formula updated successfully at $OUTPUT_PATH"