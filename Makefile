.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack ./clickhouse
	substreams pack ./erc20
	substreams pack ./erc20-rpc
	substreams pack ./erc20-contracts
	substreams pack ./native
	substreams pack ./prices-uniswap-v2

.PHONY: protogen
protogen:
	buf generate
