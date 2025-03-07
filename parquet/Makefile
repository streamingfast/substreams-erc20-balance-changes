.PHONY: all
all:
	make parquet

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run eth.substreams.pinax.network:443 ../erc20/substreams.yaml map_events "./out" 21525891:21625891 --encoder parquet --file-block-count 10000
