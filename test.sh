#!/usr/bin/env bash

cerbos run --set=storage.disk.directory=tests/testdata/policies --set=auxData.jwt.disableVerification=true -- cargo test --test sdk_test
