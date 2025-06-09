use bincode;
use risc0_zkvm::{Receipt, sha::Digest};
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Generating example proof files...");

    // Create test_files directory if it doesn't exist
    fs::create_dir_all("test_files")?;

    // Create a simple receipt (this is just for demonstration)
    let receipt = Receipt::new(
        Digest::from([0u8; 32]),
        Digest::from([0u8; 32]),
        vec![],
    );

    // Save the proof
    let proof_bytes = bincode::serialize(&receipt)?;
    fs::write("test_files/example.proof", &proof_bytes)?;

    // Create a dummy method ID (32 bytes of zeros)
    let method_id = [0u8; 32];
    fs::write("test_files/example.method_id", &method_id)?;

    println!("Example proof generated successfully!");
    println!("Files created:");
    println!("- test_files/example.proof");
    println!("- test_files/example.method_id");
    println!("\nYou can now verify these files using:");
    println!("cargo run --bin verify_example");

    Ok(())
}