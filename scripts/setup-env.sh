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
rustup install nightly-2024-12-15
rustup default nightly-2024-12-15

# Agregar WASM target si no existe
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Agregando target WASM..."
    rustup target add wasm32-unknown-unknown
fi

# Verificar que rust-toolchain.toml existe
if [ ! -f rust-toolchain.toml ]; then
    echo "Creando rust-toolchain.toml..."
    cat > rust-toolchain.toml << EOF
[toolchain]
channel = "nightly-2024-12-15"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
EOF
fi

if ! command -v cargo-stylus &> /dev/null; then
    echo "Instalando cargo-stylus..."
    cargo install cargo-stylus
fi

echo "Ambiente configurado correctamente!"
echo "Versión de Rust: $(rustc --version)"
echo "Stylus CLI: $(cargo stylus --version)"