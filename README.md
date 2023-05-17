# Rust Crypter
x86-64 Crypter built in Rust for Windows with Anti-VM, powered by memexec

## Usage
1. Put your Portable Executable in /crypt/ and rename it to example.exe (or change the code to be the same name as your PE)
2. In /crypt/ `cargo run` 
(will output encrypted_bytes.bin and key.txt)
3. move encrypted_bytes.bin and key.txt to /stub/src/
4. In /stub/ `cargo build`
5. compiled exe will be in /stub/target/debug/ named "stub.exe"

### Supported targets
- Windows x86-64
- Windows x86

### Limitations
- .NET not supported
- Files over 600MB not supported

## TODO
- File dialogue choose file instead of renaming code strings/executable names
- Automatically move encrypted bytes and key into stub for compiling
- GUI

## Disclaimer
This is a tool used to test the Dynamic detection capabilites of AV and EDR, use of this project is at your own ris

## References
https://crates.io/crates/memexec
https://crates.io/crates/inside-vm
