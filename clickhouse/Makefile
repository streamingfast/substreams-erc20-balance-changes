.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack

.PHONY: gui
gui: build
	substreams gui substreams.yaml -e eth.substreams.pinax.network:443 -s 1397553 --network eth

.PHONY: run
run: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 1397553: --undo-buffer-size 100

.PHONY: dev
dev: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e eth.substreams.pinax.network:443 21525891: --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: dev-base
dev-base: build
	substreams-sink-sql run clickhouse://default:default@localhost:9000/default substreams.yaml -e base.substreams.pinax.network:443 12860900:12862900 --final-blocks-only --undo-buffer-size 1 --on-module-hash-mistmatch=warn --batch-block-flush-interval 1 --development-mode

.PHONY: setup
setup: build
	substreams-sink-sql setup clickhouse://default:default@localhost:9000/default substreams.yaml

.PHONY: cache
cache: build
	substreams run -e eth.substreams.pinax.network:443 ./substreams.yaml db_out -s 1 -t 0 --production-mode --noop-mode