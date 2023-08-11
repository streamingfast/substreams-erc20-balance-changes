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
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run map_balance_changes -e mainnet.eth.streamingfast.io:443 -s -1000 -o jsonl

.PHONY: gui
gui: build
	substreams gui map_balance_changes -e mainnet.eth.streamingfast.io:443 -s 17829400 -t 17829410

.PHONY: deploy
deploy:
	graph deploy --studio erc-20
