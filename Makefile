.PHONY: setup build test deploy clean

setup:
	@./scripts/setup-env.sh

build:
	@cd contracts && cargo build --release --target wasm32-unknown-unknown

test:
	@cd contracts && cargo test

deploy-sepolia:
	@./scripts/deploy.sh sepolia

deploy-mainnet:
	@./scripts/deploy.sh mainnet

export-abi:
	@./scripts/export-abi.sh

clean:
	@cd contracts && cargo clean

watch:
	@cd contracts && cargo watch -x "build --release --target wasm32-unknown-unknown"

dev: build watch

check:
	@cd contracts && cargo stylus check