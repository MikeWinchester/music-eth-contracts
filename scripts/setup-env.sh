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

# Instalar la versión específica de Rust requerida por Stylus
echo "Configurando versión específica de Rust..."
rustup install nightly-2024-12-16
rustup default nightly-2024-12-16

# Instalar componentes necesarios
echo "Instalando componentes de Rust..."
rustup component add rust-src --toolchain nightly-2024-12-16
rustup component add rustfmt --toolchain nightly-2024-12-16
rustup component add clippy --toolchain nightly-2024-12-16

# Agregar WASM target
echo "Agregando target WASM..."
rustup target add wasm32-unknown-unknown --toolchain nightly-2024-12-16

if ! command -v cargo-stylus &> /dev/null; then
    echo "Instalando cargo-stylus..."
    cargo install --force cargo-stylus
fi

echo "Ambiente configurado correctamente!"
echo "Versión de Rust: $(rustc --version)"
echo "Stylus CLI: $(cargo stylus --version)"