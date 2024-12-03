#!/bin/bash

if [ "$1" = "clean" ]; then
    rm -rf target/
    exit 0
fi

# Building the OS kernel
cargo build --release

# Move object file to build directory
mkdir -p build
rm -rf build/
mkdir -p build
mv target/release/libspaghettos_kernel_rs.a build/