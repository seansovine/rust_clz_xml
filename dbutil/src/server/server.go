package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/emptypb"

	dbu "db-util/src/lib"

	pb "db-util/dbutilserver"
)

var (
	port    int = 50051
	db_host     = flag.String("db_host", "mariadb", "Database hostname.")
)

// gRPC server implementation.

type dbUtilServer struct {
	pb.UnimplementedDbUtilServer

	dbc *dbu.DbConnection
}

func (s *dbUtilServer) ResetData(context.Context, *emptypb.Empty) (*pb.CommandResult, error) {
	fmt.Println("Handling ResetData command...")

	err := s.dbc.EmptyDb()

	if err != nil {
		return &pb.CommandResult{Success: false, Error: err.Error()}, err
	}

	return &pb.CommandResult{Success: true}, nil
}

func (s *dbUtilServer) ResetSchema(context.Context, *emptypb.Empty) (*pb.CommandResult, error) {
	fmt.Println("Handling ResetSchema command...")

	err := s.dbc.ResetDb()

	if err != nil {
		return &pb.CommandResult{Success: false, Error: err.Error()}, err
	}

	return &pb.CommandResult{Success: true}, nil
}

func (s *dbUtilServer) closeConnection() {
	s.dbc.Close()
}

// Simple factor method.

func newServer() (*dbUtilServer, error) {
	dbc, err := dbu.NewDb(*db_host)

	if err != nil {
		return nil, err
	}

	return &dbUtilServer{dbc: dbc}, nil
}

func main() {
	fmt.Println("Starting dbutil gRPC server...")

	flag.Parse()

	// We use ":port" because inside the container we aren't only listening on localhost.
	// Thanks to: https://stackoverflow.com/a/64901258/3791169

	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	var opts []grpc.ServerOption

	grpcServer := grpc.NewServer(opts...)

	server, err := newServer()

	if err != nil {
		log.Fatal(err)
	}

	defer server.closeConnection()

	pb.RegisterDbUtilServer(grpcServer, server)
	grpcServer.Serve(lis)
}
