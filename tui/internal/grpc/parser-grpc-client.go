package grpc

import (
	"context"
	"fmt"
	"io"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	"tui/internal/data"

	pb "tui/clz_xml_rpc"
)

var serverAddr = "localhost:10000"

func makeClient() (*pb.ClzXmlClient, func() error, error) {
	var opts []grpc.DialOption
	opts = append(opts, grpc.WithTransportCredentials(insecure.NewCredentials()))

	conn, err := grpc.NewClient(serverAddr, opts...)
	if err != nil {
		return nil, nil, err
	}

	client := pb.NewClzXmlClient(conn)

	return &client, conn.Close, nil
}

func sendError(ch chan<- any, err error) {
	errMsg := fmt.Sprintf("gRPC call failed with error: %v", err)
	ch <- ParserError{message: errMsg}
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

	// TODO: Not sure if we need cancel here, given `defer closer()` below.
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Set up the gRPC connection.

	client, closer, err := makeClient()
	if err != nil {
		sendError(outChan, err)

		return
	}

	defer func() {
		err := closer()
		if err != nil {
			sendError(outChan, err)
		}
	}()

	file := pb.File{Path: "clz_data_sample.xml"}
	stream, err := (*client).Parse(ctx, &file)
	if err != nil {
		sendError(outChan, err)

		return
	}

	// Call the streaming endpoint until EOF RXed.

Loop:
	for {
		record, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			sendError(outChan, err)

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

		// Add authors.

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
