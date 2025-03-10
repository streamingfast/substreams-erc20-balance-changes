.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: gui
gui: build
	substreams gui substreams.yaml graph_out -e eth.substreams.pinax.network:443 --network eth -s 21525891

.PHONY: cache
cache: build
	substreams run -e eth.substreams.pinax.network:443 ./substreams.yaml graph_out -s 1 -t 0 --production-mode --noop-mode