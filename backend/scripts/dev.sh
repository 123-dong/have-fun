#!/bin/zsh

cargo run --bin gateway 2>&1 | tee /dev/tty &
cargo run --bin user 2>&1 | tee /dev/tty &

wait
