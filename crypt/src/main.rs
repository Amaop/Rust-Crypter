use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes128;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::fs::read;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Read input file into a vector RENAME example.exe to what you want and put it in this "crypt" folder
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Run with {} <inputfile.exe>", args.get(0).unwrap());
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file input not found",
        ));
    }
    let fname = args.get(1).unwrap();
    let plaintext_bytes = read(fname).expect("Failed to read file");

    // Create output files with consistent naming
    let mut encrypted_file = File::create("encrypted_Input.bin")?;
    let mut key_file = File::create("key.txt")?;

    // Define block size, in this case AES-128
    let block_size = 16;

    // Pad the bytes
    let padding_size = block_size - (plaintext_bytes.len() % block_size);
    let mut padded_plaintext_bytes = plaintext_bytes.clone();
    padded_plaintext_bytes.extend(vec![padding_size as u8; padding_size]);

    // Gen cipher with a key using nonce token
    let mut nonce = [0u8; 16];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut nonce);
    let key = GenericArray::from_slice(&nonce);

    let cipher = Aes128::new(&key);

    // Encrypt the bytes in blocks
    let mut enc_bytes = Vec::new();
    for block in padded_plaintext_bytes.chunks(block_size) {
        let mut block_array = GenericArray::clone_from_slice(block);
        cipher.encrypt_block(&mut block_array);
        enc_bytes.extend_from_slice(&block_array);
    }

    encrypted_file.write_all(&enc_bytes)?;
    key_file.write_all(&key)?;
    Ok(())
}
