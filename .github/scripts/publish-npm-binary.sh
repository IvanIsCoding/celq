#!/usr/bin/env bash
set -euo pipefail

# Usage: publish-npm-binary.sh <build-name> <build-os> <target>
# Example: publish-npm-binary.sh linux-x64-glibc ubuntu-24.04 x86_64-unknown-linux-gnu

BUILD_NAME="$1"
BUILD_OS="$2"
TARGET="$3"

BIN="celq"
NPM_DIR="npm"

# Read release version from package.json using celq
RELEASE_VERSION=$(cargo run -- "this.version" < "${NPM_DIR}/celq/package.json")
RELEASE_VERSION=${RELEASE_VERSION#\"}
RELEASE_VERSION=${RELEASE_VERSION%\"}

# Derive OS and architecture from build name
# Format: os-arch-variant (e.g., linux-x64-glibc, darwin-arm64)
node_os=$(echo "$BUILD_NAME" | cut -d '-' -f1)
node_arch=$(echo "$BUILD_NAME" | cut -d '-' -f2)

# Set package name (use 'windows' instead of 'win32')
if [[ "$BUILD_OS" == windows-* ]]; then
  node_pkg="${BIN}-windows-${node_arch}"
else
  node_pkg="${BIN}-${node_os}-${node_arch}"
fi


echo "Publishing ${node_pkg} version ${RELEASE_VERSION}"

# Create package directory
mkdir -p "${NPM_DIR}/${node_pkg}/bin"

# Generate package.json using celq
cargo run -- \
  -n \
  -p \
  -S \
  --from-file "${NPM_DIR}/package.json.cel" \
  --arg="node_os:string=${node_os}" \
  --arg="node_arch:string=${node_arch}" \
  --arg="node_version:string=${RELEASE_VERSION}" \
  --arg="node_pkg:string=${node_pkg}" \
  > "${NPM_DIR}/${node_pkg}/package.json"

# Copy binary (add .exe extension for Windows)
binary_name="$BIN"
if [[ "$BUILD_OS" == windows-* ]]; then
  binary_name="${BIN}.exe"
fi

cp "target/${TARGET}/release/${binary_name}" "${NPM_DIR}/${node_pkg}/bin/"
cp "LICENSE-MIT" "${NPM_DIR}/${node_pkg}/"
cp "LICENSE-APACHE" "${NPM_DIR}/${node_pkg}/"
cp "README.md" "${NPM_DIR}/${node_pkg}/"

# Publish package
cd "${NPM_DIR}/${node_pkg}"
npm publish --access public

echo "Successfully published ${node_pkg}"