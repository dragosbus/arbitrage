#!/bin/bash
export RUSTUP_TOOLCHAIN=stable
export LD_LIBRARY_PATH="/home/dragos/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH"
export PATH="/home/dragos/.cargo/bin:$PATH"
exec /home/dragos/.cargo/bin/rust-analyzer "$@"
