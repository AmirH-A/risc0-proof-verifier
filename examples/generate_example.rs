use bincode;
use risc0_zkvm::{Receipt, InnerReceipt, Journal, ReceiptClaim, MaybePruned, ExitCode};
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Generating example proof files...");

    fs::create_dir_all("test_files")?;

    let mock_receipt = Receipt {
        inner: InnerReceipt::Fake {
            claim: ReceiptClaim {
                pre: MaybePruned::Pruned(risc0_zkvm::sha::Digest::from([0u8; 32])),
                post: MaybePruned::Pruned(risc0_zkvm::sha::Digest::from([0u8; 32])),
                exit_code: ExitCode::Halted(0),
                input: risc0_zkvm::sha::Digest::from([0u8; 32]),
                output: MaybePruned::Pruned(risc0_zkvm::sha::Digest::from([0u8; 32])),
            },
        },
        journal: Journal::new(vec![]),
    };

    let proof_bytes = bincode::serialize(&mock_receipt)?;
    fs::write("test_files/example.proof", &proof_bytes)?;

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