build-console:
	go build -o bin/db-util src/console.go

run:
	@./bin/db-util

build_run: build-console run

proto:
	protoc --go_out=. --go_opt=paths=source_relative \
    	--go-grpc_out=. --go-grpc_opt=paths=source_relative \
		--proto_path=/usr/local/include/:. \
    	dbutilserver/dbutil.proto
