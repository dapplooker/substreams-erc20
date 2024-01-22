DSN ?= psql://postgres:root@localhost:5432/substreams_e?&schema="stack_os_data"&sslmode=disable
ENDPOINT ?= bnb.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s 12296777 -t +100 -o json

.PHONY: create_db
create_db: 
	substreams-sink-sql setup "$(DSN)" sink/substreams.eth.yaml

.PHONY: sink_db_out
sink_db_out: build
	substreams-sink-sql run "$(DSN)" sink/substreams.eth.yaml 12296288:17942367

.PHONY: sink_db_out_polygon
sink_db_out_polygon: build
	substreams-sink-sql run "$(DSN)" sink/substreams.polygon.yaml 21710495:46448222

.PHONY: sink_db_out_bnb
sink_db_out_bnb: build
	substreams-sink-sql run "$(DSN)" sink/substreams.bnb.yaml 12296288:17942367