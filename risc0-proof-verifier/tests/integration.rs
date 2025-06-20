use risc0_proof_verifier::{verify_proof_files, verify_proof_with_method_id, VerificationError};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, InnerReceipt, sha::Digest};
use risc0_zkvm::host::receipt::Claim;
use std::path::Path;

#[test]
fn test_verify_proof_with_mock_data() {
    use risc0_proof_verifier::verify_proof_with_method_id;
    let fake_proof = vec![0u8; 100];
    let fake_method_id = [0u8; 32];
    let result = verify_proof_with_method_id(&fake_proof, &fake_method_id);
    println!("result: {:?}", result);
    assert!(result.is_err());
}

#[test]
fn test_verify_dummy_proof_files() {
    use risc0_proof_verifier::verify_proof_files;
    let proof_path = "test_files/example.proof";
    let method_id_path = "test_files/example.method_id";
    let result = verify_proof_files(proof_path, method_id_path);
    println!("result: {:?}", result);
    assert!(result.is_err());
}

#[test]
fn test_verify_nonexistent_files() {
    let result = verify_proof_files("nonexistent.proof", "nonexistent.method_id");
    assert!(matches!(result, Err(VerificationError::FileLoadError(_))));
}

#[test]
fn test_verify_invalid_method_id() {
    let temp_dir = tempdir().unwrap();
    let proof_path = temp_dir.path().join("test.proof");
    let method_id_path = temp_dir.path().join("test.method_id");
    
    let mut file = File::create(&method_id_path).unwrap();
    file.write_all(&[0u8; 16]).unwrap();
    
    File::create(&proof_path).unwrap();
    
    let result = verify_proof_files(&proof_path, &method_id_path);
    assert!(matches!(result, Err(VerificationError::MethodIdFormatError(_))));
}

#[test]
fn test_verify_invalid_proof_format() {
    let temp_dir = tempdir().unwrap();
    let proof_path = temp_dir.path().join("test.proof");
    let method_id_path = temp_dir.path().join("test.method_id");
    
    let mut file = File::create(&method_id_path).unwrap();
    file.write_all(&[0u8; 32]).unwrap();
    
    let mut file = File::create(&proof_path).unwrap();
    file.write_all(&[0u8; 100]).unwrap();
    
    let result = verify_proof_files(&proof_path, &method_id_path);
    assert!(result.is_err());
}

#[test]
fn test_verify_proof_with_method_id_invalid_proof() {
    let invalid_proof = vec![0u8; 100]; // Invalid proof bytes
    let method_id_bytes = [0u8; 32];
    
    let result = verify_proof_with_method_id(&invalid_proof, &method_id_bytes);
    assert!(result.is_err());
}

#[test]
fn test_verify_proof_with_method_id_invalid_verification() {
    let temp_dir = tempdir().unwrap();
    let proof_path = temp_dir.path().join("test.proof");
    let method_id_path = temp_dir.path().join("test.method_id");
    
    let mut file = File::create(&method_id_path).unwrap();
    file.write_all(&[0u8; 32]).unwrap();
    
    let mut file = File::create(&proof_path).unwrap();
    let dummy_data = vec![0u8; 100];
    file.write_all(&dummy_data).unwrap();
    
    let result = verify_proof_files(&proof_path, &method_id_path);
    assert!(result.is_err());
}