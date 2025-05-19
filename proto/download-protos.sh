#!/usr/bin/env bash
#
# Copyright 2021 Zenauth Ltd.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROTO_DIR="${SCRIPT_DIR}/defs"
CERBOS_MODULE=${CERBOS_MODULE:-"buf.build/cerbos/cerbos-api"}
CLOUD_MODULE=${CLOUD_MODULE:-"buf.build/cerbos/cloud-api"}

rm -rf "$PROTO_DIR"
(
    cd "$SCRIPT_DIR"
    buf export "$CERBOS_MODULE" --output="${PROTO_DIR}"
    buf export "$CLOUD_MODULE" --output="${PROTO_DIR}" --path cerbos/cloud/store/v1
)
