.PHONY: all
all:
	make build
	make pack
	make graph
	make info


.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release


.PHONY: pack
pack: build
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run: pack
	substreams run ./erc20-balance-changes-v0.0.1.spkg map_balance_changes -e mainnet.eth.streamingfast.io:443 -s 15000000 -t 16000000 --output jsonl
.PHONY: gui
gui:
	substreams gui ./erc20-balance-changes-v0.0.1.spkg map_valid_changes -e mainnet.eth.streamingfast.io:443 -s 17920000 --production-mode

