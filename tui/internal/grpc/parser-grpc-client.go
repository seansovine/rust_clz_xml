package grpc

import (
	"context"
	"fmt"
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

type ParserError struct {
	message string
}

// Implement error interface.
func (e ParserError) Error() string {
	return e.message
}

func Parser(outChan chan<- any, controlChan <-chan any) {
	defer close(outChan)

	// TODO: Note sure if we need cancel here, given `defer closer()` below.
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Call the streaming endpoint.

	client, closer := makeClient()
	defer closer()

	file := pb.File{Path: "clz_data_sample.xml"}
	stream, err := (*client).Parse(ctx, &file)
	if err != nil {
		errMsg := fmt.Sprintf("gRPC call failed with error: %v", err)
		outChan <- ParserError{message: errMsg}

		return
	}

Loop:
	for {
		record, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			errMsg := fmt.Sprintf("gRPC receive failed with error: %v", err)
			outChan <- ParserError{message: errMsg}

			break
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

		select {
		case <-controlChan:
			break Loop

		case outChan <- bookRecord:
			// Nothing else to do here.
		}
	}
}
