#!/bin/sh

cargo init
cargo run
cargo build
cargo clean
cargo bench

cargo search yaml
cargo install yaml-rust2

