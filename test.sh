#!/usr/bin/env bash

cerbos run --set=storage.disk.directory=resources/store -- cargo test --test sdk_test
