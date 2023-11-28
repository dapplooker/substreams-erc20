ENDPOINT ?= arb-one.streamingfast.io:443

.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: tt
tt: 
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s 22207900 -t +100000 -o json

.PHONY: gi
gi: 
	substreams gui -e $(ENDPOINT) substreams.yaml graph_out -s 0 -t +100

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

