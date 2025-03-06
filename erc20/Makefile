.PHONY: all
all:
	make build
	substreams info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events -e eth.substreams.pinax.network:443 --network eth -s 21525891

.PHONY: cache
cache: build
	substreams-sink-noop eth.substreams.pinax.network:443 ./substreams.yaml map_events 1:
