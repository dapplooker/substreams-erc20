DSN ?= psql://postgres:G25xZVFg9tpr5kbCQpFu@dapplooker-production.cgrozmytutpw.us-east-1.rds.amazonaws.com:5432/dapplooker_dapp_production?&schema="stack_os_data"&sslmode=require&sslrootcert=/root/.ssh/us-east-1-bundle.pem
ENDPOINT ?= bnb.streamingfast.io:443

.PHONY: build
build:
	cargo +stable build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s 12296777 -t +100 -o json

.PHONY: create_db
create_db:
	substreams-sink-sql setup "$(DSN)" sink/substreams.eth.yaml

# .PHONY: sink_db_out
# sink_db_out: build
# 	substreams-sink-sql run "$(DSN)" sink/substreams.eth.yaml 12296288:18035164
#
# .PHONY: sink_db_out_polygon
# sink_db_out_polygon: build
# 	substreams-sink-sql run "$(DSN)" sink/substreams.polygon.yaml 21710495:46448222

.PHONY: sink_db_out_bnb
sink_db_out_bnb: build
	substreams-sink-sql run "$(DSN)" sink/substreams.bnb.yaml 31335612:35323560
