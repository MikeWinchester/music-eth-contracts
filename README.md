# Descargar e instalar rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cargar el ambiente de Rust
source ~/.cargo/env

# Usar Rust stable (recomendado para Stylus)
rustup default stable

# Instalar target para WebAssembly
rustup target add wasm32-unknown-unknown

# Instalar componentes adicionales
rustup component add rustfmt clippy rust-src

# Instalar cargo-stylus (herramienta principal)
cargo install cargo-stylus

# Verificar instalación
cargo stylus --version

# cargo-watch (para desarrollo automático)
cargo install cargo-watch

# cargo-audit (para auditoría de seguridad)
cargo install cargo-audit

# Foundry (para testing e interacción con contratos)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Configurar todo de una vez
chmod +x scripts/*.sh
make setup

# Compilar y verificar
make build
make check

# Desarrollo con hot reload
make dev

# Deploy rápido a testnet
make deploy-sepolia

# Exportar ABI para el frontend
make export-abi