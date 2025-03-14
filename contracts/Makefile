.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run base.substreams.pinax.network:443 ./substreams.yaml map_events "./out" 24400000: --encoder parquet --file-block-count 1000

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events -e eth.substreams.pinax.network:443 --network eth -s 21525891

.PHONY: gui-base
gui-base: build
	substreams gui substreams.yaml map_events -e base.substreams.pinax.network:443 --network base -s 24400000 -t 0pp