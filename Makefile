.PHONY: all
all:
	make build
	make pack
	make graph
	make info


.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release

.PHONY: pack
pack: build
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: build
	substreams run substreams.yaml map_balance_changes -e mainnet.eth.streamingfast.io:443 -s 15000000 -t 16000000 --output jsonl

.PHONY: gui
gui: build
	substreams gui substreams.yaml balance_change_stats -e mainnet.eth.streamingfast.io:443 --production-mode --network eth -s 21841000 -t +1

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events "./out" 18005793:18005794 --encoder parquet --file-block-count 1 --development-mode
