build:
	go build -o bin/main cmd/tui/main.go

run: ./bin/main
	./bin/main

proto:
	protoc --go_out=. --go_opt=paths=source_relative \
    	--go-grpc_out=. --go-grpc_opt=paths=source_relative \
		--proto_path=/usr/local/include/:. \
    	clz_xml_rpc/clz_xml.proto

build_grpc_test:
	go build -o bin/grpc_test_client cmd/grpc_test_client/grpc_test_client.go

run_grpc_test: ./bin/grpc_test_client
	./bin/grpc_test_client
