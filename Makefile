.PHONY: all
all:
	make build
	substreams info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: run
run: build
	substreams run substreams.yaml map_balance_changes -e mainnet.eth.streamingfast.io:443 -s 15000000 -t 16000000 --output jsonl

.PHONY: gui
gui: build
	substreams gui substreams.yaml balance_change_stats -e mainnet.eth.streamingfast.io:443 --production-mode --network eth -s 21841000 -t +1

.PHONY: sql
sql: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 21529220:21529235 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: sql-setup
sql-setup: build
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events "./out" 21529220:21529235 --encoder parquet --file-block-count 1 --development-mode
