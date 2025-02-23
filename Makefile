.PHONY: all
all:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: run
run: build
	substreams run substreams.yaml map_balance_changes -e mainnet.eth.streamingfast.io:443 -s 15000000 -t 16000000 --output jsonl

.PHONY: gui
gui: build
	substreams gui substreams.yaml balance_change_stats -e mainnet.eth.streamingfast.io:443 --production-mode --network eth -s 21841000 -t +1

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 substreams.yaml map_events "./out" 21529220:21529235 --encoder parquet --file-block-count 1 --development-mode
