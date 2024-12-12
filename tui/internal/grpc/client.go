package grpc

import (
	"context"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"io"
	"log"

	"tui/internal/data"

	pb "tui/clz_xml_rpc"
)

var serverAddr = "[::1]:10000"

func makeClient() (*pb.ClzXmlClient, func()) {
	var opts []grpc.DialOption
	opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))

	conn, err := grpc.NewClient(serverAddr, opts...)
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	closer := func() {
		err := conn.Close()
		if err != nil {
			log.Fatalf("close failed: %v", err)
		}
	}

	client := pb.NewClzXmlClient(conn)

	return &client, closer
}

func Parser(ch chan<- any) {
	defer close(ch)

	// Call the streaming endpoint.

	client, closer := makeClient()

	// NOTE: We will want to be able to cancel long-running
	// parse operations, so we add this context.
	// TODO: Add a cancellation feature in the UI.
	ctx, cancel := context.WithCancel(context.Background())

	defer func() {
		cancel()
		closer()
	}()

	file := pb.File{Path: ""}
	stream, err := (*client).Parse(ctx, &file)
	if err != nil {
		log.Fatalf("%v.Parse RPC call failed with error: %v", client, err)
	}

	for {
		record, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalf("%v.Parse(_) command failed with error: %v", client, err)
		}
		ch <- data.BookRecord{Title: record.Title}
	}

	ch <- "Done"
}
