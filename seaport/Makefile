.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 ./substreams.yaml map_events "./out" 20686058: --encoder parquet --file-block-count 100 --development-mode

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events -e eth.substreams.pinax.network:443 --network eth -s 20686703

