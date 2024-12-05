#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

OS_FILE=target/x86_64_spaghettos/debug/bootimage-spaghettos_kernel_rs.bin

# Build with cargo and `bootimage` library
cargo bootimage --target x86_64_spaghettos.json

qemu-system-x86_64 -display sdl -drive format=raw,file="$OS_FILE" -d cpu_reset -d int -no-reboot -no-shutdown