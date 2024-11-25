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

build_docker:
	BUILDKIT_PROGRESS=plain docker build . -f dbutil.dockerfile -t dbutil-grpc-server

run_docker:
	@ docker run -p 50051:50051 dbutil-grpc-server

# For debugging the build inside the container.
docker_shell: SHELL:=/bin/bash
docker_shell:
	docker run --name dbutil-grpc-server -dit docker.io/library/dbutil-grpc-server /bin/sh
	docker exec -it dbutil-grpc-server sh \
		&& docker stop dbutil-grpc-server
