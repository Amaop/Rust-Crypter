#!/bin/bash

# Simple batch script that follows the exact README steps for each file
# Usage: ./simple_batch.sh /path/to/folder/with/exe/files

INPUT_FOLDER="$1"
OUTPUT_DIR="batch_output"

if [[ -z "$INPUT_FOLDER" ]]; then
    echo "Usage: $0 /path/to/folder/with/exe/files"
    exit 1
fi

if [[ ! -d "$INPUT_FOLDER" ]]; then
    echo "Error: Folder '$INPUT_FOLDER' does not exist"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "Processing files from: $INPUT_FOLDER"
echo "Output directory: $OUTPUT_DIR"

# Process each file
for file in "$INPUT_FOLDER"/*; do
    if [[ ! -f "$file" ]]; then
        continue
    fi
    
    # Skip files with extensions other than .exe or no extension
    if [[ "$file" == *.* && "$file" != *.exe ]]; then
        continue
    fi
    
    filename=$(basename "$file")
    basename="${filename%.*}"
    
    echo "Processing: $filename"
    
    # Step 1: Copy file to /crypt/
    cp "$file" crypt/
    
    # Step 2: Run cargo run in /crypt/
    echo "  Encrypting..."
    cd crypt
    cargo run "$filename"
    cd ..
    
    # Step 3: Move encrypted files to /stub/src/
    mv crypt/encrypted_Input.bin stub/src/
    mv crypt/key.txt stub/src/
    
    # Step 4: Build the stub
    echo "  Building stub..."
    cd stub
    cargo build --target x86_64-pc-windows-gnu --release
    cd ..
    
    # Step 5: Copy the compiled exe to output directory
    cp stub/target/x86_64-pc-windows-gnu/release/stub.exe "$OUTPUT_DIR/${basename}_encrypted.exe"
    
    # Clean up
    rm -f crypt/"$filename"
    
    echo "  ✓ Created: $OUTPUT_DIR/${basename}_encrypted.exe"
done

echo "Batch processing complete! Check $OUTPUT_DIR for results."
