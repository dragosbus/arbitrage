#!/bin/bash
# Script to restart rust-analyzer with proper environment
echo "Setting up Rust environment..."
export RUSTUP_TOOLCHAIN=stable
export LD_LIBRARY_PATH="/home/dragos/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH"
export PATH="/home/dragos/.cargo/bin:$PATH"

echo "Testing rust-analyzer..."
rust-analyzer --version

echo "Rust environment setup complete!"
echo "If you're still having issues in VS Code, try:"
echo "1. Restart VS Code completely"
echo "2. Run 'Developer: Reload Window' command in VS Code"
echo "3. Check that rust-analyzer extension is enabled"
