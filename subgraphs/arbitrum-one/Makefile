.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node=http://localhost:8020 erc20
	graph deploy --node=http://localhost:8020 erc20

.PHONY: cache
cache:
	substreams-sink-noop arbone.substreams.pinax.network:443 ../../ graph_out 0:

.PHONY: publish
publish:
	make build
	graph publish --subgraph-id '<INSERT SUBGRAPH ID>''