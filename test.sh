#!/usr/bin/env bash

cerbos run --set=storage.disk.directory=resources/store --set=auxData.jwt.disableVerification=true -- cargo test --test sdk_test
