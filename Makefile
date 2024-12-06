import:
	cd parser; \
	cargo run --package clz_data --bin clz_data -- "../data/clz_data_sample.xml"

build_grpc:
	cargo build --color=always --package clz_data --bin parser_grpc --profile dev

run_grpc:
	cargo run --package clz_data --bin parser_grpc

