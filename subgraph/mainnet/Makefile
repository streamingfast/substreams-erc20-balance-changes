.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 erc20-eth
	graph deploy --node=http://localhost:8020 erc20-eth

.PHONY: cache
cache:
	substreams-sink-noop eth.substreams.pinax.network:443 ../ graph_out 1:

.PHONY: publish
publish:
	make build
	graph publish --subgraph-id 7aw3rrZz7uhLFVQ7LDhM8BYCh59AFKqaiG9rxShwzVB8