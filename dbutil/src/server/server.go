package main

import (
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"

	pb "db-util/dbutilserver"
)

var port int = 50051

type dbUtilServer struct {
	pb.UnimplementedDbUtilServer
}

func newServer() *dbUtilServer {
	return &dbUtilServer{}
}

func main() {
	lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	var opts []grpc.ServerOption

	grpcServer := grpc.NewServer(opts...)
	pb.RegisterDbUtilServer(grpcServer, newServer())
	grpcServer.Serve(lis)
}
