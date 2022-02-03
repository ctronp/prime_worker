#!/usr/bin/bash

set -e

cd "$(dirname "$0")" || exit
cargo build --release --target x86_64-unknown-linux-musl
cp target/release/api bootstrap
zip bootstrap.zip bootstrap
rm bootstrap