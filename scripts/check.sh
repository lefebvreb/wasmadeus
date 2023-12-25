#!/usr/bin/env sh

# This is the poor man's CI pipeline; runs fmt, clippy and miri test

set -e

function suppress {
    OUTPUT=`$* --color=always 2>&1` || echo $OUTPUT
}

echo cargo fmt
cargo fmt

echo cargo clippy
suppress cargo clippy
suppress cargo +nightly clippy --features nightly

echo cargo miri test
suppress cargo +nightly miri test
suppress cargo +nightly miri test --features nightly

echo cargo doc
suppress cargo +nightly doc --no-deps --all-features

echo all done!
