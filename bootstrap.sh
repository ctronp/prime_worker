#!/usr/bin/bash

cd "$(dirname "$0")" || exit
cross build --release  --target aarch64-unknown-linux-gnu
cp target/release/api bootstrap