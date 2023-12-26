#!/usr/bin/env sh

# This is the poor man's CI pipeline; runs fmt, clippy, miri test and rustdoc

set -e

function suppress {
    OUTPUT=`$* 2>&1` || echo $OUTPUT
}

echo cargo fmt
cargo fmt

echo cargo clippy
suppress cargo clippy --color=always
suppress cargo +nightly clippy --features nightly --color=always

echo cargo miri test
suppress cargo +nightly miri test --color=always
suppress cargo +nightly miri test --features nightly --color=always

echo cargo rustdoc
suppress cargo +nightly rustdoc --all-features -- --cfg doc_cfg

echo all done!
