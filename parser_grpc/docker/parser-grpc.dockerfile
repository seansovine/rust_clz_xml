## Stage one: Build executable.

FROM rust:bookworm AS builder

# Note we need to keep the build and run distro
# versions in sync or we may have library version
# trouble at runtime.

# Install protobuf compiler.
RUN apt-get update \
	&& DEBIAN_FRONTEND=noninteractive \
	apt-get install --no-install-recommends --assume-yes \
	protobuf-compiler

WORKDIR /usr/src/parser_grpc

# gRPC server source
COPY ./parser_grpc .

# XML parser library source
COPY ./parser /usr/src/parser

RUN cargo install --path .

## Stage two: Copy executable and run it.

FROM debian:bookworm-slim

# In case we need to install runtime dependencies.
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/parser_grpc /usr/local/bin/parser_grpc

WORKDIR /parser_grpc

CMD ["parser_grpc"]
