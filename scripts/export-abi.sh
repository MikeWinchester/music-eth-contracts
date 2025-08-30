#!/bin/bash

NETWORK=${1:-sepolia}

echo "Exportando ABI para red: $NETWORK"

cd ../contracts

# Crear directorio si no existe
if [ -d "../abis" ]; then
    echo "Folder exists"
else
    mkdir -p ../abis
    echo "Folder created"
fi

# Compilar con feature export-abi
echo "Compilando con export-abi feature..."
cargo build --features export-abi --target wasm32-unknown-unknown

# Exportar ABI
echo "Exportando ABI..."
cargo stylus export-abi --output ../abis/MusicStreaming.json

if [ $? -eq 0 ]; then
    echo "ABI exportado a: abis/MusicStreaming.json"
    
    # Si existe deployment info, combinar con ABI
    DEPLOYMENT_FILE="../deployments/$NETWORK.json"
    if [ -f $DEPLOYMENT_FILE ]; then
        # Verificar que jq esté instalado
        if command -v jq &> /dev/null; then
            jq -s '.[0] + {"abi": .[1]}' $DEPLOYMENT_FILE ../abis/MusicStreaming.json > ../abis/MusicStreaming-$NETWORK.json
            echo "Archivo combinado creado: abis/MusicStreaming-$NETWORK.json"
        else
            echo "jq no está instalado, saltando combinación de archivos"
        fi
    fi
else
    echo "Error exportando ABI"
    exit 1
fi