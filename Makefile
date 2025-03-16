.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack ./
	substreams pack ./erc20
	substreams pack ./native
	substreams pack ./contracts
	substreams pack ./clickhouse
	substreams pack ./subgraph

.PHONY: protogen
protogen:
	buf generate

.PHONY: gui
gui: build
	substreams gui substreams.yaml db_out -e eth.substreams.pinax.network:443 -s 1 -t 0 --network eth --production-mode

.PHONY: noop
noop: build
	substreams-sink-noop eth.substreams.pinax.network:443 substreams.yaml db_out --state-store state.eth.yaml

.PHONY: noop-base
noop-base: build
	substreams-sink-noop base.substreams.pinax.network:443 substreams.yaml db_out --state-store state.base.yaml