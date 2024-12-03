#!/usr/bin/env just --justfile

fmt:
  cargo +nightly fmt

lint:
  cargo clippy --features=full