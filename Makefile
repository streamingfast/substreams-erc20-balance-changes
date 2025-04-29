.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack ./clickhouse
	substreams pack ./clickhouse-ens
	substreams pack ./clickhouse-nfts
