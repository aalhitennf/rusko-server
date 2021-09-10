#!/bin/sh
cargo build --release
strip ./target/release/rusko
mkdir -p build
cp ./target/release/rusko ./build/rusko