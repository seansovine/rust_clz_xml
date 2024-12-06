use crate::clz_xml::clz_xml_server::ClzXml;
use crate::clz_xml::{BookRecord, File};
use clz_data::data::MainMessage;

use crate::parser_thread::parser_thread_main;

use colored::Colorize;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ClzXmlService;

#[tonic::async_trait]
impl ClzXml for ClzXmlService {
  /// Server streaming response type for the Parse method.
  type ParseStream = ReceiverStream<Result<BookRecord, Status>>;

  async fn parse(&self, _request: Request<File>) -> Result<Response<Self::ParseStream>, Status> {
    println!("Starting parse response to client! (Asynchronously.)");

    let (tx, rx) = tokio::sync::mpsc::channel(4);

    let _test_books = vec![
      BookRecord {
        title: String::from("War and Peace"),
        year: None,
        isbn: None,
        publisher: None,
        authors: vec![],
      },
      BookRecord {
        title: String::from("Batman"),
        year: None,
        isbn: None,
        publisher: None,
        authors: vec![],
      },
    ];

    tokio::spawn(async move {
      run_parser(tx).await;
    });

    Ok(Response::new(ReceiverStream::new(rx)))
  }
}

/// Runs the parser thread and reads its results.
async fn run_parser<T>(tx: Sender<Result<BookRecord, T>>) {
  let parser_control = parser_thread_main().unwrap();

  let parser_tag = "PARSER".yellow();

  let mut books_found = vec![];

  // Read books until parser channel sends WorkComplete.
  for message in &parser_control.receiver {
    match message {
      MainMessage::ParserData(book) => {
        println!(
          ">> {parser_tag}: UID {}: Found book with title: '{}'",
          book.uid, book.title
        );

        let book_record = BookRecord {
          title: book.title,
          year: None,
          isbn: None,
          publisher: None,
          authors: vec![],
        };

		books_found.push(book_record);
      }

      MainMessage::ParserWorkComplete => {
        println!("\n -- {} --\n", "Parser Finished.".green());
      }

      MainMessage::ParserGeneric(message) => {
        println!(">> {parser_tag}: {}", message);
      }

      _ => panic!("Main received unexpected message type!"),
    }
  }

  let _parse_result = parser_control.handle.join().unwrap();

  // NOTE: We would like to send these back to the client as
  // they are received from the parser, as that would take full
  // advantage of async. However, the compiler does not like us
  // awaiting tx.send within the loop over the receiver.
  //
  // It seems the reason is that it includes the receiver in the
  // context of its promise, but that receiver is a std::sync::mpsc
  // receiver and not a tokio::sync::mpsc receiver, so it doesn't
  // implement Send.
  //
  // We could fix this by using async mpsc's in our parser. But,
  // that might entail async-ifying much of the rest of that code.
  // That would be OK. We'll work on that when we get around to it.
  // For now the task doesn't start sending results until the parser
  // is finished running.

  for book_record in &books_found {
	tx.send(Ok(book_record.clone())).await.unwrap();
  }
}
