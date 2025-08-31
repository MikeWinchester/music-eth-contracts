#!/bin/bash

set -e

NETWORK=${1:-sepolia}

echo "Exportando ABI para red: $NETWORK"

# Crear directorios
mkdir -p ../abis

cd ../contracts

# Compilar y extraer ABI manualmente
echo "Compilando contrato..."
cargo build --release

# Buscar el archivo de artefacto de compilación
ARTIFACT_DIR="./target/release"
CONTRACT_NAME="MusicStreamingPlatform"  # Ajusta según tu contrato

# Verificar si existe el archivo de artefacto
if [ -f "$ARTIFACT_DIR/$CONTRACT_NAME.json" ]; then
    echo "Extrayendo ABI del artefacto..."
    # Extraer solo la parte del ABI
    jq '.abi' "$ARTIFACT_DIR/$CONTRACT_NAME.json" > ../abis/MusicStreaming.json
    echo "ABI exportado a: ../abis/MusicStreaming.json"
else
    echo "Buscando archivos de artefacto..."
    find "$ARTIFACT_DIR" -name "*.json" | head -5
    echo "No se encontró el artefacto esperado"
    exit 1
fi

# Combinar con deployment info si existe
DEPLOYMENT_FILE="../deployments/$NETWORK.json"
if [ -f "$DEPLOYMENT_FILE" ]; then
    if command -v jq &> /dev/null; then
        echo "Combinando con deployment info..."
        jq -s '.[0] + {abi: .[1]}' "$DEPLOYMENT_FILE" ../abis/MusicStreaming.json > ../abis/MusicStreaming-$NETWORK.json
        echo "Archivo combinado creado: ../abis/MusicStreaming-$NETWORK.json"
    else
        echo "jq no está instalado, saltando combinación"
    fi
else
    echo "No se encontró deployment file: $DEPLOYMENT_FILE"
fi

echo "Proceso completado"