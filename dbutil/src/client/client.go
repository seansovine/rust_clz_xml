package main

import (
	"context"
	"fmt"
	"log"
	"time"

	pb "db-util/dbutilserver"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	emptypb "google.golang.org/protobuf/types/known/emptypb"
)

var serverAddr = "127.0.0.1:50051"

func main() {
	fmt.Println("Starting dbutil gRPC client...")

	var opts []grpc.DialOption
	opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))

	conn, err := grpc.NewClient(serverAddr, opts...)

	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer conn.Close()

	client := pb.NewDbUtilClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()

	result, err := client.ResetData(ctx, &emptypb.Empty{})

	if err != nil {
		log.Fatal(err)
	}

	if result.Success {
		fmt.Println("Reset data command success!")
	} else {
		log.Fatal(result.Error)
	}
}
