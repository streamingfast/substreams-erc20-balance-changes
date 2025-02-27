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

.PHONY: gui
gui: build
	substreams gui substreams.yaml map_events -e eth.substreams.pinax.network:443 --network eth -s 21525891

.PHONY: production
production: build
	substreams gui substreams.yaml map_events -e eth.substreams.pinax.network:443 --network eth -s 21525891 -t 0 --production-mode