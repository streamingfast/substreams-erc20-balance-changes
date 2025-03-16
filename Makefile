.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack ./clickhouse
	substreams pack ./erc20
	substreams pack ./native
	substreams pack ./contracts

.PHONY: protogen
protogen:
	buf generate
