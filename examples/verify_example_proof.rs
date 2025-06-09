use risc0_proof_verifier::verify_proof_files;

fn main() -> anyhow::Result<()> {
    println!("Verifying example proof...");
    
    let result = verify_proof_files("test_files/example.proof", "test_files/example.method_id");
    
    match result {
        Ok(()) => {
            println!("Proof verification successful!");
            println!("This is expected to fail in this example because we used dummy data.");
            println!("In a real application, you would use actual proofs and method IDs.");
        }
        Err(e) => {
            println!("Proof verification failed as expected:");
            println!("Error: {}", e);
            println!("\nThis is normal because we used dummy data for demonstration.");
            println!("In a real application, you would use actual proofs and method IDs.");
        }
    }

    Ok(())
} 