#!/bin/bash

NETWORK=${1:-sepolia}

echo "Exportando ABI para red: $NETWORK"

cd contracts

# Exportar ABI
cargo stylus export-abi > ../abis/MusicStreaming.json

if [ $? -eq 0 ]; then
    echo "ABI exportado a: abis/MusicStreaming.json"
    
    # Si existe deployment info, combinar con ABI
    DEPLOYMENT_FILE="../deployments/$NETWORK.json"
    if [ -f $DEPLOYMENT_FILE ]; then
        # Crear archivo combinado para el frontend
        jq -s '.[0] + {"abi": .[1]}' $DEPLOYMENT_FILE ../abis/MusicStreaming.json > ../abis/MusicStreaming-$NETWORK.json
        echo "Archivo combinado creado: abis/MusicStreaming-$NETWORK.json"
    fi
else
    echo "Error exportando ABI"
    exit 1
fi