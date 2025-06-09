# risc0-proof-verifier

A Rust library and example for verifying RISC Zero zkVM proofs.

## Project Structure

- `src/` - Library code for proof verification
- `guest/` - Minimal RISC Zero guest program (Fibonacci)
- `examples/generate_proof.rs` - Host program to generate a proof
- `test_files/` - Example proof and method ID files for testing
- `tests/` - Integration and unit tests

## How to Build and Test

### 1. Build the Guest Program

```sh
cd guest
cargo build --release --target riscv32im-risc0-zkvm-elf
```

### 2. Generate a Proof and Method ID

```sh
cd ../examples
cargo run --release --example generate_proof

# Generate the method ID (requires RISC Zero CLI)
cargo install --locked risczero-cli # if not already installed
cargo risczero method-id --elf ../guest/target/riscv32im-risc0-zkvm-elf/release/guest > ../test_files/fib.method_id
```

### 3. Run the Tests

```sh
cd .. # project root
cargo test
```

## Notes
- The proof and method ID files are saved in `test_files/` and used by the integration tests.
- The guest program computes the nth Fibonacci number (n=10 by default).

## License
MIT OR Apache-2.0 