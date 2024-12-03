#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Set script variables
PROJECT_DIR="$(pwd)"
KERNEL_NAME="spaghettos_kernel_rs"
BUILD_DIR="${PROJECT_DIR}/build"

# Navigate to the kernel project directory
cd "$PROJECT_DIR"

# Ensure the build directory exists
mkdir -p "$BUILD_DIR"

cargo build --release --target x86_64-unknown-none -q 

# Path to the compiled kernel
KERNEL_LIB="${PROJECT_DIR}/target/x86_64-unknown-none/release/lib${KERNEL_NAME}.a"

# Check if the compiled kernel exists
if [ ! -f "$KERNEL_LIB" ]; then
    echo "Error: Compiled kernel not found at $KERNEL_LIB"
    exit 1
fi

# Copy the kernel binary to the build directory
cp "$KERNEL_LIB" "$BUILD_DIR/libkernel.a"