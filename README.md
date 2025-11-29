# Rust Crypter
x86-64 Malware Crypter built in Rust for Windows with Anti-VM, powered by memexec

## Overview

A proof-of-concept crypter that encrypts Windows PE executables and embeds them in a loader stub for fileless execution. The crypter uses AES encryption with anti-analysis features and is designed for security research and AV/EDR evasion testing.

## How It Works

### Architecture
- **`crypt/`** - Encryption component that processes input PE files
- **`stub/`** - Loader component that decrypts and executes payloads in memory
- **`simple_batch.sh`** - Automation script for batch processing multiple files

### Encryption Process
1. **Input**: Windows PE executable (.exe)
2. **Encryption**: 
   - Uses AES-128 encryption from Rust crypto crates
   - Applies PKCS#7 padding for block alignment
   - Generates random 16-byte key per encryption
   - Produces `encrypted_Input.bin` and `key.txt`
3. **Packaging**: Encrypted payload and key are embedded into stub binary
4. **Output**: Compiled stub executable for Windows x86-64 target

### Runtime Behavior
- **Anti-Analysis**: VM detection using `inside-vm` crate
- **Persistence**: Registry Run key creation and hidden directory setup
- **Decryption**: AES decryption of embedded payload using stored key
- **Execution**: Fileless payload execution via `memexec` crate (reflective loading)

### Security Features
- **Non-deterministic**: Each encryption produces different output (randomized keys/IVs)
- **Memory-only execution**: No decrypted payload written to disk
- **Evasion techniques**: Anti-VM checks and obfuscated execution flow

## Usage

### Single File
1. Put your .exe in `/crypt/`
2. `cd crypt && cargo run <filename.exe>`
3. `mv encrypted_Input.bin key.txt ../stub/src/`
4. `cd ../stub && cargo build --target x86_64-pc-windows-gnu --release`
5. Your encrypted exe is in `stub/target/x86_64-pc-windows-gnu/release/stub.exe`

### Batch Processing (Multiple Files)
```bash
./simple_batch.sh /path/to/folder/with/exe/files
```
Output: `batch_output/` folder with `{filename}_encrypted.exe` files

### Supported Targets
- Windows x86-64
- Windows x86

### Limitations
- .NET assemblies not supported
- Files over 600MB not supported
- Requires Windows target compilation environment

## Technical Details

### Dependencies
- **AES Encryption**: `aes`, `block-modes`, `block-padding` crates
- **Memory Execution**: `memexec` crate for reflective loading
- **Anti-VM**: `inside-vm` crate for environment detection
- **Cross-compilation**: Targets `x86_64-pc-windows-gnu`

### Output Characteristics
- Each encryption run produces unique binaries (non-deterministic)
- Hash comparison between identical inputs will show different results
- Encrypted payloads are embedded as binary resources in stub

## TODO
- File dialogue choose file instead of renaming code strings/executable names
- Automatically move encrypted bytes and key into stub for compiling
- GUI
- Obfuscated Strings

## Security & Ethics

⚠️ **IMPORTANT DISCLAIMER**: This tool is designed for authorized security research and testing AV/EDR detection capabilities. Use only in controlled lab environments with proper authorization. Misuse for malicious purposes is prohibited and may violate laws.

### Responsible Use Guidelines
- Only use on systems you own or have explicit permission to test
- Follow applicable laws and organizational policies
- Do not distribute malicious payloads
- Use for defensive security research and education only

## MITRE ATT&CK TTPs (Indicators)
- **T1204.002** - User Execution: Malicious File
- **T1140** - Deobfuscate/Decode Files or Information
- **T1027.009** - Embedded Payloads
- **T1497.001** - System Checks (Anti-VM)
- **T1620** - Reflective Code Loading
- **T1547.001** - Registry Run Keys Persistence

## References
- [memexec crate](https://crates.io/crates/memexec)
- [inside-vm crate](https://crates.io/crates/inside-vm)
- [AES implementation](https://crates.io/crates/aes)