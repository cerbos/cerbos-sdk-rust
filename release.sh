#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <release_version>"
    exit 2
fi

REMOTE=${REMOTE:-"upstream"}
RELEASE_VER="${1#v}"

cargo install cargo-release
cargo release --sign --no-publish --push-remote "$REMOTE" --verbose --execute "$RELEASE_VER"
