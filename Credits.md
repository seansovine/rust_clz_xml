# Sources and Credits

I've found a lot of great information online while working on this so far.
I will mention some of the sources I drew from the most directly here.

**Use of `quick-xml`:**

Our use of this library for XML parsing was inspired by [this](https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/)
blog post.

**gRPC Setup:**

We based our gRPC setup on the basic Go example in the docs [here](https://grpc.io/docs/languages/go/basics/).

**Dockerfile for Golang server:**

We based our gRPC server Dockerfile on this example in
[this](https://hackernoon.com/running-a-grpc-service-with-docker) tutorial.

**Deno + React + Vite + React Table:**

We started with [this](https://github.com/denoland/react-vite-ts-template) Deno
example project with React and Vite, and Dockerized it for use with our Compose
application.

Then we added a React Table component that we modified from
[this](https://tanstack.com/table/latest) example.
