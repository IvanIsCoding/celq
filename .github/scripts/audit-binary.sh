#!/bin/sh

set -e

BINARY="$1"

if [ -z "$BINARY" ]; then
  echo "Usage: $0 <path-to-binary>"
  exit 2
fi

if [ ! -f "$BINARY" ]; then
  echo "Error: binary does not exist: $BINARY"
  exit 2
fi

if cargo audit bin "$BINARY"; then
  exit 0
else
  audit_status=$?
fi

if [ "$audit_status" -eq 1 ]; then
  echo "::warning::cargo-audit found one or more vulnerabilities in the binary"
else
  exit "$audit_status"
fi
