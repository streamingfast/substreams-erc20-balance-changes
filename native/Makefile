.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 ../erc20/substreams.yaml map_events "./out" 21525891:21525892 --encoder parquet --file-block-count 1

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events -e eth.substreams.pinax.network:443 --network eth -s 21525891

.PHONY: cache
cache: build
	substreams-sink-noop eth.substreams.pinax.network:443 ./substreams.yaml map_events 1:
