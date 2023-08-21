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
run: build
	substreams run map_valid_changes -e mainnet.eth.streamingfast.io:443
.PHONY: gui
gui:
	#substreams gui ./erc20-balance-changes-v0.0.1.spkg map_balance_changes -e mainnet.eth.streamingfast.io:443 -s 17920000 -t +10 --production-mode
	#substreams gui ./erc20-balance-changes-v0.0.1.spkg map_valid_changes -e mainnet.eth.streamingfast.io:443 -s 13320000 --production-mode
	substreams gui ./erc20-balance-changes-v0.0.1.spkg map_valid_changes -e mainnet.eth.streamingfast.io:443 -s 17920000 --production-mode

