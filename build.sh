#!bin/bash
mkdir -p release

cargo build --release
cargo build --release --target x86_64-pc-windows-gnu

cp target/release/honeycomb release/honeycomb
cp target/x86_64-pc-windows-gnu/release/honeycomb.exe release/honeycomb.exe