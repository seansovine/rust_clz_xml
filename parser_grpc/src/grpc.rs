use crate::clz_xml::clz_xml_server::{ClzXml, ClzXmlServer};
use crate::clz_xml::{AuthorRecord, BookRecord, File};
use clz_data::data::MainMessage;

use crate::parser_thread::parser_thread_main;

use std::pin::Pin;
use std::sync::Arc;

use colored::Colorize;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct ClzXmlService;

#[tonic::async_trait]
impl ClzXml for ClzXmlService {
  /// Server streaming response type for the Parse method.
  type ParseStream = ReceiverStream<Result<BookRecord, Status>>;

  async fn parse(&self, request: Request<File>) -> Result<Response<Self::ParseStream>, Status> {
    unimplemented!()
  }
}

/// Runs the parser thread and reads its results.
fn _run_parser() {
	let parser_control = parser_thread_main().unwrap();

	let parser_tag = "PARSER".yellow();

	// Read books until parser channel sends WorkComplete.
	for message in &parser_control.receiver {
	  match message {
		MainMessage::ParserData(book) => {
		  println!(
			">> {parser_tag}: UID {}: Found book with title: '{}'",
			book.uid, book.title
		  );
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
}
