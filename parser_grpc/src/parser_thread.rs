use std::fs::File;
use std::io::BufReader;
use std::thread::{self, JoinHandle};

use tokio::sync::mpsc::{self, Receiver};

use clz_data::data::MainMessage;
use clz_data::parse;

use quick_xml::reader::Reader;

const CHANNEL_BUFFER_SIZE: usize = 1000;

pub struct ParserControl {
  pub handle: JoinHandle<Result<(), std::io::Error>>,
  // Channel receiver to parser thread from us.
  pub receiver: Receiver<MainMessage>,
}

pub fn parser_thread_main() -> Result<ParserControl, Box<dyn std::error::Error>> {
  let file_path = "../data/clz_data_sample.xml";
  let file = File::open(file_path)?;

  // Create buffered reader.
  let reader = BufReader::new(file);
  // Create quick-xml reader.
  let reader = Reader::from_reader(reader);

  // Channel from parser worker to main.
  let (main_sender, main_receiver) = mpsc::channel::<MainMessage>(CHANNEL_BUFFER_SIZE);

  // Spawn the parser thread.
  let parser_handle = thread::spawn(move || parse::read_xml(reader, main_sender));

  return Ok(ParserControl {
    handle: parser_handle,
    receiver: main_receiver,
  });
}
