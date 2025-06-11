use bincode;
use risc0_zkvm::{Receipt, sha::Digest};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Failed to load file: {0}")]
    FileLoadError(String),
    #[error("Invalid proof format: {0}")]
    ProofFormatError(String),
    #[error("Invalid MethodId format: {0}")]
    MethodIdFormatError(String),
    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),
}

pub fn verify_proof_with_method_id(
    proof_bytes: &[u8],
    method_id_bytes: &[u8; 32],
) -> Result<(), VerificationError> {
    let receipt: Receipt = bincode::deserialize(proof_bytes)
        .map_err(|e| VerificationError::ProofFormatError(format!("{e}")))?;

    println!("Receipt: {:?}", receipt);

    receipt
        .verify(Digest::from(*method_id_bytes))
        .map_err(|e| VerificationError::ProofVerificationFailed(format!("{e}")))?;

    Ok(())
}

pub fn verify_proof_files<P1: AsRef<Path>, P2: AsRef<Path>>(
    proof_path: P1,
    method_id_path: P2,
) -> Result<(), VerificationError> {
    let proof_bytes = fs::read(&proof_path)
        .map_err(|e| VerificationError::FileLoadError(format!("proof: {e}")))?;
    let method_id_vec = fs::read(&method_id_path)
        .map_err(|e| VerificationError::FileLoadError(format!("method_id: {e}")))?;
    let method_id: &[u8; 32] = method_id_vec
        .as_slice()
        .try_into()
        .map_err(|_| VerificationError::MethodIdFormatError("method_id must be 32 bytes".into()))?;

    verify_proof_with_method_id(&proof_bytes, method_id)
}