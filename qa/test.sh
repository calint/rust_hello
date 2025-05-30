#!/bin/sh
set -e
cd $(dirname "$0")

cd ..
cargo build
cd qa

echo -e "$(cat test.in)" | ../target/debug/rust_hello > test.out

if cmp -s test.diff test.out; then
    echo "test: PASSED"
    rm test.out
else
    echo "test: FAILED, check 'diff test.diff test.out'"
fi