.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 ../erc20/substreams.yaml map_events "./out" 21525891:21525892 --encoder parquet --file-block-count 1

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events -e base.substreams.pinax.network:443 --network eth -s 12861427 -t +1

.PHONY: cache
cache: build
	substreams run -e eth.substreams.pinax.network:443 ./substreams.yaml map_events -s 1 -t 0 --production-mode --noop-mode
