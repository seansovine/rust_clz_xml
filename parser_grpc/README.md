# Rust gRPC Service for CLZ XML Parser

This has an endpoint that takes a filename and
then runs the parser on that file and streams any
records it finds while parsing back to the client.
The TUI will soon use this to allow the user to
perform an in-the-loop process of updating the database,
where the TUI displays records that the user can approve
or reject before inserting them. However, there are
other ways this can be used, and that flexibility is
a nice result of making this a gRPC service.

## Developer details

__Managing `.proto` files:__

The `build.rs` script includes a step for building the `.proto`
file to produce the protobuf and gRPC structures needed by
the server. Right now we just copy the same proto file to the
projects where we define a client. It would be better later
to keep the `.proto` files shared by multiple sub-projects in
a common folder, and to just modify the appropriate build scripts
to look there. That way we avoid duplication and having to keep
things in sync.

## Docker build and run

There is a Dockerfile for a container to build and run the parser
gRPC serve in the `docker` subfolder of this folder. You can build
the image by

```shell
cd $PROJECT_ROOT
docker build -t parser-grpc -f parser_grpc/docker/parser-grpc.dockerfile .
```

and run the server in the container with

```shell
docker run -it --rm --mount type=bind,src=./data/,dst=/data \
	-p 10000:10000 --name parser-grpc-service parser-grpc
```

And you can stop it with

```shell
docker stop parser-grpc-service
```

We've also added this as a `parser-grpc` service in the Docker Compose app:

```shell
docker compose up parser-grpc
```
