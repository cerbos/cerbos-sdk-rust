set dotenv-load := true

default:
    @ just --choose

lint:
    @ cargo check
    @ cargo clippy --all-features

tests: test-sdk test-admin-sdk test-hub-sdk

test-sdk-local:
    #!/usr/bin/env bash
    set -euo pipefail

    cerbos run --set=storage.disk.directory=tests/testdata/policies --set=auxData.jwt.disableVerification=true -- cargo test --test sdk_test

test-sdk:
    @ cargo test --features testcontainers --test sdk_test

test-admin-sdk:
    @ cargo test --features admin,serde,testcontainers --test sdk_admin_test

test-hub-sdk:
    @ cargo test --features hub --test store_integration_test -- --test-threads=1


