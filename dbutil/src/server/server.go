package main

import (
	"context"
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/emptypb"

	dbu "db-util/src/lib"

	pb "db-util/dbutilserver"
)

var port int = 50051

// gRPC server implementation.

type dbUtilServer struct {
	pb.UnimplementedDbUtilServer
}

func (s *dbUtilServer) ResetData(context.Context, *emptypb.Empty) (*pb.CommandResult, error) {
	err := dbu.EmptyDb()

	if err != nil {
		return &pb.CommandResult{Success: false}, err
	}

	return &pb.CommandResult{Success: true}, nil
}

func (s *dbUtilServer) ResetSchema(context.Context, *emptypb.Empty) (*pb.CommandResult, error) {
	err := dbu.ResetDb()

	if err != nil {
		return &pb.CommandResult{Success: false}, err
	}

	return &pb.CommandResult{Success: true}, nil
}

// Simple factor method.

func newServer() *dbUtilServer {
	return &dbUtilServer{}
}

func main() {
	fmt.Println("Starting dbutil gRPC server...")

	lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	var opts []grpc.ServerOption

	grpcServer := grpc.NewServer(opts...)
	pb.RegisterDbUtilServer(grpcServer, newServer())
	grpcServer.Serve(lis)
}
