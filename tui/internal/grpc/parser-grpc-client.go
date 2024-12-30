package grpc

import (
	"context"
	"io"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	"tui/internal/data"

	pb "tui/clz_xml_rpc"
)

var serverAddr = "localhost:10000"

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

func Parser(ctx context.Context, ch chan<- any) {
	defer close(ch)

	// Call the streaming endpoint.

	client, closer := makeClient()
	defer closer()

	file := pb.File{Path: "clz_data_sample.xml"}
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

		bookRecord := data.BookRecord{
			Title: record.Title,
		}

		// Add optional fields.

		descriptor := record.ProtoReflect().Descriptor()
		yearField := descriptor.Fields().ByTextName("year")
		isbnField := descriptor.Fields().ByTextName("isbn")
		publisherField := descriptor.Fields().ByTextName("publisher")

		if record.ProtoReflect().Has(yearField) {
			year := record.GetYear()
			bookRecord.Year = &year
		}
		if record.ProtoReflect().Has(isbnField) {
			isbn := record.GetIsbn()
			bookRecord.Isbn = &isbn
		}
		if record.ProtoReflect().Has(publisherField) {
			publisher := record.GetPublisher()
			bookRecord.Publisher = &publisher
		}

		// Add authors

		for _, author := range record.GetAuthors() {
			firstName := author.GetFirstName()
			lastName := author.GetLastName()
			bookRecord.Authors = append(bookRecord.Authors, data.AuthorRecord{
				FirstName: &firstName,
				LastName:  &lastName,
			})
		}

		// Now send the record to Bubbletea goroutine.
		ch <- bookRecord
	}
}
