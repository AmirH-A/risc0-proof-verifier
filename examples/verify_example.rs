use risc0_proof_verifier::{verify_proof_files, VerificationError};
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Verifying example proof...");
    
    // Check if the files exist
    if !fs::metadata("test_files/example.proof").is_ok() {
        println!("Error: test_files/example.proof not found. Please run generate_example first.");
        return Ok(());
    }
    if !fs::metadata("test_files/example.method_id").is_ok() {
        println!("Error: test_files/example.method_id not found. Please run generate_example first.");
        return Ok(());
    }

    // Verify the proof
    let result = verify_proof_files("test_files/example.proof", "test_files/example.method_id");
    
    match result {
        Ok(()) => {
            println!("Proof verification successful!");
            println!("The proof was verified against the method ID.");
        }
        Err(e) => {
            println!("Proof verification failed:");
            match e {
                VerificationError::FileLoadError(msg) => {
                    println!("File error: {}", msg);
                }
                VerificationError::ProofFormatError(msg) => {
                    println!("Invalid proof format: {}", msg);
                }
                VerificationError::MethodIdFormatError(msg) => {
                    println!("Invalid method ID format: {}", msg);
                }
                VerificationError::ProofVerificationFailed(msg) => {
                    println!("Proof verification failed: {}", msg);
                    println!("\nNote: This is expected with the example proof as it uses dummy data.");
                    println!("In a real application, you would use actual proofs and method IDs.");
                }
            }
        }
    }

    Ok(())
} 