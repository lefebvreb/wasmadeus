#!/usr/bin/env sh

# This is the poor man's CI pipeline; runs fmt, clippy and miri test

set -e

function suppress {
    OUTPUT=`$* --color=always 2>&1` || echo "$OUTPUT"
}

echo cargo fmt
cargo fmt

echo cargo clippy
suppress cargo clippy
suppress cargo clippy --features nightly

echo cargo miri test
suppress cargo miri test
suppress cargo miri test --features nightly

echo all done!
