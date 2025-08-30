.PHONY: setup build test deploy clean

# Cargar variables de entorno
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

# Usar endpoint por defecto si no está definido
RPC_URL ?= https://sepolia-rollup.arbitrum.io/rpc

setup:
	@./scripts/setup-env.sh

build:
	@./scripts/build.sh

build-clean:
	@./scripts/build.sh --clean

test:
	@cd contracts && cargo test

deploy-sepolia:
	@cd contracts && ../scripts/deploy.sh sepolia

deploy-mainnet:
	@cd contracts && ../scripts/deploy.sh mainnet

export-abi:
	@cd contracts && ../scripts/export-abi.sh

clean:
	@cd contracts && cargo clean

watch:
	@cd contracts && cargo watch -x "build --release --target wasm32-unknown-unknown"

dev: build watch

check: build
	@cd contracts && cargo stylus check --endpoint $(RPC_URL) --wasm-file target/wasm32-unknown-unknown/release/music_streaming_contracts.wasm

check-local:
	@cd contracts && cargo stylus check --wasm-file target/wasm32-unknown-unknown/release/music_streaming_contracts.wasm