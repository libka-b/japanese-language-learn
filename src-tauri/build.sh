#!/bin/sh

# check formatting
cargo fmt --all -- check

# linting
cargo clippy -- -D warnings

# tests
cargo test

# build
cargo build
