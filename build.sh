#!/bin/sh
cargo build --release
strip ./target/release/rusko
mkdir -p build/
mkdir -p build/rusko-v$1
cp ./target/release/rusko ./build/rusko-v$1/rusko
cd ./build
tar -czvf ./rusko-v$1.tar.gz ./rusko-v$1
