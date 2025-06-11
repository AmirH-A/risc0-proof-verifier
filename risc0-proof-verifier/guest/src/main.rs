#![no_std]
#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::entry!(main);

fn main() {
    // Read the input from the host
    let input: Vec<u8> = env::read();
    
    // Echo the input back
    env::commit(&input);
} 