#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <package> <version>"
    exit 2
fi

REMOTE=${REMOTE:-"upstream"}
PACKAGE="$1"
VERSION="${2#v}"

cargo install cargo-release
cargo release --package "$PACKAGE" --sign --no-publish --push-remote "$REMOTE" --verbose --execute "$VERSION"
