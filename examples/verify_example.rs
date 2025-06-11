use risc0_proof_verifier::{verify_proof_files, VerificationError};
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Verifying proof... for testi");
    
    if !fs::metadata("test_files/proof.bin").is_ok() {
        println!("Error: test_files/proof.bin not found.");
        return Ok(());
    }
    if !fs::metadata("test_files/method_id.bin").is_ok() {
        println!("Error: test_files/method_id.bin not found.");
        return Ok(());
    }

    let result = verify_proof_files("test_files/proof.bin", "test_files/method_id.bin");
    
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
                }
            }
        }
    }

    Ok(())
} 