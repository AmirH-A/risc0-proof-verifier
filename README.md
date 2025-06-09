# RISC Zero Proof Verifier

A library for verifying RISC Zero proofs.

## Features

#### Verify proofs using method IDs
####  File-based proof verification
#### Example code for generating and verifying proofs

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
risc0-proof-verifier = "0.1.0"
```

## Examples

### Generating Example Proof Files

To generate example proof files for testing:

```bash
# Using the script
./scripts/generate_example.sh

# Or manually
mkdir -p test_files
dd if=/dev/zero of=test_files/example.proof bs=1 count=100
dd if=/dev/zero of=test_files/example.method_id bs=1 count=32
```

This will create:
- `test_files/example.proof` (100 bytes of zeros)
- `test_files/example.method_id` (32 bytes of zeros)

### Verifying Proofs

To verify a proof:

```rust
use risc0_proof_verifier::{verify_proof_files, verify_proof_with_method_id};

// Verify from files
let result = verify_proof_files("path/to/proof", "path/to/method_id")?;

// Or verify directly with bytes
let proof_bytes = std::fs::read("path/to/proof")?;
let method_id_bytes = std::fs::read("path/to/method_id")?;
let method_id: &[u8; 32] = method_id_bytes.as_slice().try_into()?;
let result = verify_proof_with_method_id(&proof_bytes, method_id)?;
```

## Testing

The project includes comprehensive tests for various scenarios:

1. **Integration Tests**:
   ```bash
   cargo test
   ```
   Tests include:
   - Verifying with mock data
   - Verifying with dummy proof files
   - Handling nonexistent files
   - Handling invalid method IDs
   - Handling invalid proof formats

2. **Example Files**:
   - `test_files/example.proof`: A dummy proof file for testing
   - `test_files/example.method_id`: A dummy method ID file for testing


## Note for Apple Silicon Users

Due to limitations with the RISC Zero toolchain on Apple Silicon (arm64) Macs, generating real proofs may not work. The example files use dummy data to demonstrate the API usage. For generating real proofs, consider using a Linux environment or GitHub Codespaces.

## Project Structure

```
risc0-proof-verifier/
├── src/
│   ├── lib.rs           # Main library code
│   └── bin/             # Binary examples
├── examples/            # Example code
├── tests/              # Integration tests
├── test_files/         # Example proof files
└── scripts/            # Helper scripts
```

## What works
- All host-side code and tests are implemented and pass using mock (dummy) proof and method ID data.
- The verification logic is fully exercised and ready for integration with real proofs when the toolchain supports your platform.

## How to run tests
```sh
cargo test
```

## If you need to generate real proofs
- Use an x86-64 Linux machine or GitHub Codespaces to build the guest and generate proofs.
- Copy the generated files into `test_files/` and rerun the tests.

--- 