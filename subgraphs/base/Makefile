.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 erc20-base
	graph deploy --node=http://localhost:8020 erc20-base

.PHONY: cache
cache:
	substreams-sink-noop base.substreams.pinax.network:443 ../ graph_out 1:

.PHONY: publish
publish:
	make build
	graph publish --subgraph-id ...