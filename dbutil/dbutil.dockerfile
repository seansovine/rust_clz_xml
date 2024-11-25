# Modified from example here:: https://hackernoon.com/running-a-grpc-service-with-docker

FROM golang:alpine

# Install git and ca-certificates for HTTPS
RUN apk --update add ca-certificates git

WORKDIR /app

COPY ./dbutilserver ./dbutilserver
COPY ./src ./src
COPY go.mod .
COPY go.sum .

# Download module dependencies
RUN go mod download

# Build with static linking (not sure if necessary).
RUN CGO_ENABLED=0 GOOS=linux go build -modfile ./go.mod -ldflags '-extldflags "-static"' -o main ./src/server/server.go

# Run server on container run.
CMD ["/app/main"]
