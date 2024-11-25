# Modified from example here:: https://hackernoon.com/running-a-grpc-service-with-docker

FROM golang:alpine

# Install git and ca-certificates (needed to be able to call HTTPS)
RUN apk --update add ca-certificates git

# Move to working directory /app
WORKDIR /app

# Copy the code into the container
COPY ./src .

# Download dependencies using go mod
RUN go mod download

# Build the application's binary
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -ldflags '-extldflags "-static"' -o main .
# TODO: Do we need all these flags if CGO_ENABLED=0?

# Command to run the application when starting the container
CMD ["/app/main"]
