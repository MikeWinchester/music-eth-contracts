#!/bin/bash

NETWORK=${1:-sepolia}

echo "Exportando ABI para red: $NETWORK"

cd ../contracts

if [ ! -d "../abis" ]; then
    echo "No se detecto la carpeta \"abis\""
    mkdir "../abis"
    echo "Carpeta \"abis\" creada correctamente"
fi

cargo stylus export-abi --output "../abis/MusicStreaming.json" --json

if [ $? -eq 0 ]; then
    echo "ABI exportado a: ./abis/MusicStreaming.rs"
    
    # Si existe deployment info, combinar con ABI
    # DEPLOYMENT_FILE="../deployments/$NETWORK.json"
    # if [ -f $DEPLOYMENT_FILE ]; then
    #     # Crear archivo combinado para el frontend
    #     jq -s '.[0] + {"abi": .[1]}' $DEPLOYMENT_FILE ../abis/MusicStreaming.json > ../abis/MusicStreaming-$NETWORK.json
    #     echo "Archivo combinado creado: abis/MusicStreaming-$NETWORK.json"
    # fi
else
    echo "Error exportando ABI"
    exit 1
fi