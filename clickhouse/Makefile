.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: sql
sql: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 21525891: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: sql-setup
sql-setup: build
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml

.PHONY: cache
cache: build
	substreams-sink-noop eth.substreams.pinax.network:443 ./substreams.yaml db_out 1: --state-store eth.state.yaml