.PHONY: all
all:
	make parquet

.PHONY: parquet
parquet:
	rm -f state.yaml && substreams-sink-files run base.substreams.pinax.network:443 ../erc20/substreams.yaml map_events "./out" 12860900:12862900 --encoder parquet --file-block-count 100
