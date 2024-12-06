#!/usr/bin/env just --justfile

fmt:
  cargo +nightly fmt

lint:
  cargo clippy --features=full

run day part:
  cargo run -p {{day}} --features={{part}}

run-release day part:
  cargo run --release -p {{day}} --features={{part}}

test-all part='full':
   cargo nextest run --features={{part}}

test day part='full':
   cargo nextest run -p {{day}} --features={{part}}