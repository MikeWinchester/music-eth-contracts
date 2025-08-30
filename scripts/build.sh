#!/bin/bash

# scripts/build.sh - Build helper para Stylus

set -e

echo "Building Stylus contract..."

cd contracts

# Limpiar si es necesario
if [ "$1" = "--clean" ]; then
    echo "Cleaning..."
    cargo clean
fi

# Compilar con los flags correctos
echo "Compiling..."
RUSTFLAGS="-C panic=abort" cargo build --release --target wasm32-unknown-unknown

echo "Build completed successfully!"

# Verificar que el archivo WASM existe
WASM_FILE="target/wasm32-unknown-unknown/release/music_streaming_contracts.wasm"
if [ -f "$WASM_FILE" ]; then
    echo "WASM file: $WASM_FILE ($(du -h $WASM_FILE | cut -f1))"
else
    echo "WASM file not found!"
    exit 1
fi