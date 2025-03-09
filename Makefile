.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack ./erc20
	substreams pack ./native
	substreams pack ./subgraph
	substreams pack ./clickhouse

.PHONY: protogen
protogen:
	buf generate

