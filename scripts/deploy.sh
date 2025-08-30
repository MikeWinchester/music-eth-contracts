#!/bin/bash

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para logging
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}"
}

warn() {
    echo -e "${YELLOW}[WARNING] $1${NC}"
}

# Cargar variables de entorno
if [ -f .env ]; then
    export $(cat .env | xargs)
else
    error ".env file not found!"
    exit 1
fi

# Validar variables requeridas
if [ -z "$PRIVATE_KEY" ] || [ -z "$RPC_URL" ]; then
    error "PRIVATE_KEY y RPC_URL son requeridos en .env"
    exit 1
fi

NETWORK=${1:-sepolia}

log "Desplegando en red: $NETWORK"

# Compilar contrato
log "Compilando contrato..."
cd contracts
cargo build --release --target wasm32-unknown-unknown

if [ $? -ne 0 ]; then
    error "Compilación falló!"
    exit 1
fi

# Verificar compatibilidad con Stylus
log "Verificando compatibilidad con Stylus..."
cargo stylus check --endpoint $RPC_URL
if [ $? -ne 0 ]; then
    error "Verificación de Stylus falló!"
    exit 1
fi

# Desplegar
log "Desplegando contrato..."
DEPLOY_OUTPUT=$(cargo stylus deploy \
    --wasm-file target/wasm32-unknown-unknown/release/music_streaming_contracts.wasm \
    --private-key $PRIVATE_KEY \
    --endpoint $RPC_URL \
    --no-verify)

# Verificar si el deploy fue exitoso
if [ $? -ne 0 ]; then
    error "Despliegue falló!"
    exit 1
fi

log "Contrato desplegado exitosamente!"

# Extraer dirección del contrato del output
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -i "deployed.*code.*0x" | grep -o "0x[a-fA-F0-9]\{40\}" | tail -n1)

if [ -z "$CONTRACT_ADDRESS" ]; then
    error "No se pudo extraer la dirección del contrato!"
    exit 1
fi

log "Contrato desplegado en: $CONTRACT_ADDRESS"

# Guardar deployment info
if [ -d "../deployments" ]; then
    echo "Folder exists"
else
    mkdir ../deployments
    echo "Folder does not exist"
fi
DEPLOYMENT_FILE="../deployments/$NETWORK.json"
cat > $DEPLOYMENT_FILE << EOF
{
  "network": "$NETWORK",
  "contractAddress": "$CONTRACT_ADDRESS",
  "deploymentDate": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "deployer": "$(echo $DEPLOY_OUTPUT | grep -o 'from: 0x[a-fA-F0-9]\{40\}' | cut -d' ' -f2)",
  "transactionHash": "$(echo $DEPLOY_OUTPUT | grep -o 'Transaction hash: 0x[a-fA-F0-9]\{64\}' | cut -d' ' -f3)"
}
EOF

log "Deployment info guardado en: $DEPLOYMENT_FILE"

# Exportar ABI
log "Exportando ABI..."
../scripts/export-abi.sh $NETWORK

log "¡Despliegue completado exitosamente!"
log "Dirección del contrato: $CONTRACT_ADDRESS"
log "Red: $NETWORK"