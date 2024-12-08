package main

import (
	"context"
	"io"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "tui/clz_xml_rpc"
)

var serverAddr = "[::1]:10000"

func main() {
	var opts []grpc.DialOption
	opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))

	conn, err := grpc.NewClient(serverAddr, opts...)
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer func() {
		err := conn.Close()
		if err != nil {
			log.Fatalf("close failed: %v", err)
		}
	}()

	client := pb.NewClzXmlClient(conn)

	// Call the streaming endpoint.

	file := pb.File{Path: ""}
	stream, err := client.Parse(context.Background(), &file)
	if err != nil {
		log.Fatalf("parse rpc failed: %v", err)
	}

	for {
		record, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalf("%v.Parse(_) = _, %v", client, err)
		}
		log.Println(record)
	}
}
