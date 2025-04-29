.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: gui
gui: build
	substreams gui substreams.yaml db_out -e eth.substreams.pinax.network:443 -s 22211598 --network eth

.PHONY: noop
noop: build
	substreams run substreams.yaml db_out -e eth.substreams.pinax.network:443 -s 22211598 -t 0 --limit-processed-blocks 0 --network eth --production-mode --noop-mode

.PHONY: dev
dev: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 22211598: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: setup
setup: build
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml
