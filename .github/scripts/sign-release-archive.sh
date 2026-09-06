#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <archive-path> <signature-path>" >&2
  exit 1
fi

if [ -z "${MINISIGN_PRIVATE_KEY:-}" ]; then
  echo "MINISIGN_PRIVATE_KEY must be set" >&2
  exit 1
fi

ARCHIVE_PATH="$1"
SIGNATURE_PATH="$2"

if [ ! -f "${ARCHIVE_PATH}" ]; then
  echo "Archive not found: ${ARCHIVE_PATH}" >&2
  exit 1
fi

TEMP_ROOT="${RUNNER_TEMP:-${TMPDIR:-/tmp}}"
KEY_PATH=$(mktemp "${TEMP_ROOT%/}/celq-minisign-key.XXXXXX")

cleanup() {
  rm -f "${KEY_PATH}"
}
trap cleanup EXIT

chmod 600 "${KEY_PATH}"
printf '%s' "${MINISIGN_PRIVATE_KEY}" > "${KEY_PATH}"

minisign -S -W \
  -s "${KEY_PATH}" \
  -x "${SIGNATURE_PATH}" \
  -m "${ARCHIVE_PATH}"
