#!/bin/bash
set -e

echo "Building guest program..."
cargo risczero build

echo "Generating proof..."
cargo run --example generate_proof

echo "Generating method ID..."
cargo risczero method-id --elf guest/target/riscv32im-risc0-zkvm-elf/release/guest > test_files/fib.method_id

echo "Running tests..."
cargo test

echo "All tests completed successfully!" 