#!/bin/bash
# cargo run --features=ssr --bin simple_ssr_server -- --dir dist & CARGO_PID=$!
# wait $CARGO_PID
# echo "Cargo run has completed. Starting server..."



cd ssr_server
# cargo run --features=ssr --bin simple_ssr_server -- --dir dist
cargo run --features=ssr --bin simple_ssr_server -- --dir dist &
CARGO_PID=$!

wait $CARGO_PID

echo "Cargo run has completed. Starting server..."
