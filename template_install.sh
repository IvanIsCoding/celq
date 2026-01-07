#!/usr/bin/env bash

# This work is dedicated to the public domain under CC0 1.0.
# https://creativecommons.org/publicdomain/zero/1.0/
#
# To the extent possible under law, the author has waived all
# copyright and related or neighboring rights to this work.

set -eu
# Check pipefail support in a subshell, ignore if unsupported
# shellcheck disable=SC3040
(set -o pipefail 2> /dev/null) && set -o pipefail

help() {
  cat <<'EOF'
Install a binary release of celq hosted on GitHub

USAGE:
    install.sh [options]

FLAGS:
    -h, --help      Display this message
    -f, --force     Force overwriting an existing binary

OPTIONS:
    --to LOCATION   Where to install the binary [default: ~/bin]
    --target TARGET
EOF
}

crate=celq
url=https://github.com/IvanIsCoding/celq
releases=$url/releases

say() {
  echo "install: $*" >&2
}

err() {
  if [ -n "${td-}" ]; then
    rm -rf "$td"
  fi

  say "error: $*"
  exit 1
}

need() {
  if ! command -v "$1" > /dev/null 2>&1; then
    err "need $1 (command not found)"
  fi
}

download() {
  url="$1"
  output="$2"
  args=()

  if [ -n "${GITHUB_TOKEN+x}" ]; then
    args+=(--header "Authorization: Bearer $GITHUB_TOKEN")
  fi

  if command -v curl > /dev/null; then
    curl --proto =https --tlsv1.2 -sSfL ${args[@]+"${args[@]}"} "$url" -o"$output"
  else
    wget --https-only --secure-protocol=TLSv1_2 --quiet ${args[@]+"${args[@]}"} "$url" -O"$output"
  fi
}

force=false
while test $# -gt 0; do
  case $1 in
    --force | -f)
      force=true
      ;;
    --help | -h)
      help
      exit 0
      ;;
    --target)
      target=$2
      shift
      ;;
    --to)
      dest=$2
      shift
      ;;
    *)
      say "error: unrecognized argument '$1'. Usage:"
      help
      exit 1
      ;;
  esac
  shift
done

command -v curl > /dev/null 2>&1 ||
  command -v wget > /dev/null 2>&1 ||
  err "need wget or curl (command not found)"

need mkdir
need mktemp

if [ -z "${target-}" ]; then
  need cut
fi

if [ -z "${dest-}" ]; then
  dest="$HOME/bin"
fi

if [ -z "${target-}" ]; then
  # bash compiled with MINGW (e.g. git-bash, used in github windows runners),
  # unhelpfully includes a version suffix in `uname -s` output, so handle that.
  # e.g. MINGW64_NT-10-0.19044
  kernel=$(uname -s | cut -d- -f1)
  uname_target="$(uname -m)-$kernel"

  case $uname_target in
    aarch64-Linux) target=aarch64-unknown-linux-musl;;
    arm64-Darwin) target=aarch64-apple-darwin;;
    armv6l-Linux) target=arm-unknown-linux-musleabihf;;
    armv7l-Linux) target=armv7-unknown-linux-musleabihf;;
    loongarch64-Linux) target=loongarch64-unknown-linux-musl;;
    x86_64-Darwin) target=x86_64-apple-darwin;;
    x86_64-Linux) target=x86_64-unknown-linux-musl;;
    x86_64-MINGW64_NT) target=x86_64-pc-windows-msvc;;
    x86_64-Windows_NT) target=x86_64-pc-windows-msvc;;
    *)
      # shellcheck disable=SC2016
      err 'Could not determine target from output of `uname -m`/`uname -s`, please use `--target`:' "$uname_target"
    ;;
  esac
fi

case $target in
  x86_64-pc-windows-msvc) extension=zip; need unzip;;
  *) extension=tgz; need tar;;
esac

archive="$releases/download/{{CELQ_VERSION}}/$crate-$target.$extension"

say "Repository:  $url"
say "Crate:       $crate"
say "Tag:         {{CELQ_VERSION}}"
say "Target:      $target"
say "Destination: $dest"
say "Archive:     $archive"

td=$(mktemp -d || mktemp -d -t tmp)

if [ "$extension" = "zip" ]; then
  download "$archive" "$td/celq.zip"
  unzip -d "$td" "$td/celq.zip"
else
  download "$archive" - | tar -C "$td" -xz
fi

if [ -e "$dest/celq" ] && [ "$force" = false ]; then
  err "\`$dest/celq\` already exists"
else
  mkdir -p "$dest"
  cp "$td/celq" "$dest/celq"
  chmod 755 "$dest/celq"
fi

rm -rf "$td"