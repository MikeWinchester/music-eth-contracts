#!/bin/bash

echo "Configurando ambiente de desarrollo..."

# Crear archivo .env si no existe
if [ ! -f .env ]; then
    cp .env.example .env
    echo "Archivo .env creado. Por favor configura tus variables."
fi

# Verificar instalaciones
echo "Verificando dependencias..."

if ! command -v cargo &> /dev/null; then
    echo "Rust no está instalado. Instalando..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
fi

# Agregar WASM target si no existe
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Agregando target WASM..."
    rustup target add wasm32-unknown-unknown
fi

if ! command -v cargo-stylus &> /dev/null; then
    echo "Instalando cargo-stylus..."
    cargo install cargo-stylus
fi

echo "Ambiente configurado correctamente!"