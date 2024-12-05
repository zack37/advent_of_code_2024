#!/usr/bin/env just --justfile

fmt:
  cargo +nightly fmt

lint:
  cargo clippy --features=full

run day part:
  cargo run -p {{day}} --features={{part}}

test-all:
   cargo nextest run --features=full

test day:
   cargo nextest run -p {{day}} --features=full