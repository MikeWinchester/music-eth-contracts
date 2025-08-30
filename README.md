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