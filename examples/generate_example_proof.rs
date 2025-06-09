use bincode;
use risc0_zkvm::{ExecutorEnv, default_prover, Receipt};
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("test_files")?;

    let input: u32 = 42;
    let env = ExecutorEnv::builder()
        .stdin(&input.to_le_bytes())
        .build()?;

    let receipt = Receipt::new(
        risc0_zkvm::sha::Digest::from([0u8; 32]),
        risc0_zkvm::sha::Digest::from([0u8; 32]),
        vec![],
    );

    let proof_bytes = bincode::serialize(&receipt)?;
    fs::write("test_files/example.proof", &proof_bytes)?;

    let method_id = [0u8; 32];
    fs::write("test_files/example.method_id", &method_id)?;

    println!("Example proof generated successfully!");
    println!("Files created:");
    println!("- test_files/example.proof");
    println!("- test_files/example.method_id");
    println!("\nYou can now use these files to test the verification:");
    println!("cargo run --example verify_example_proof");

    Ok(())
} 