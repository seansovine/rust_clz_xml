mod parse;
mod database;
mod data;

/// Start of a simple app to read the CLZ books XML file.
///
/// The use of quick-xml was inspired by
///     https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
/// but mostly based on the simple example from the docs,
///     https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html
///
/// A future idea is to load the data into a database. See README for discussion.

use crate::data::{DatabaseMessage, MainMessage};

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;

use quick_xml::reader::Reader;

fn main() -> std::io::Result<()> {
    // Open XML file.
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;

    // Create buffered reader.
    let reader = BufReader::new(file);
    // Create quick-xml reader.
    let reader = Reader::from_reader(reader);

    // Channel from workers to main.
    let (main_sender, main_receiver) = mpsc::channel::<MainMessage>();

    let main_sender_parser = main_sender.clone();
    // Start database thread.
    let parser_handle = thread::spawn(move || {
        parse::read_xml(reader, main_sender_parser)
    });

    // Create channel to database thread.
    let (database_sender, database_receiver) = mpsc::channel::<DatabaseMessage>();
    // Rename main sender for symmetry ;)
    let main_sender_database = main_sender;
    // Start database thread.
    let database_handle = thread::spawn(move || {
        database::database_main(database_receiver, main_sender_database);
    });

    // Read books until channel closes, at end of read_xml.
    for message in main_receiver {
        match message {
            MainMessage::Data(book) => {
                println!("Found book with title: '{}'", book.title);

                // Send the book data to the database thread.
                database_sender.send(DatabaseMessage::Data(book)).unwrap()
            }

            MainMessage::WorkComplete => break,
        }
    }

    println!("Finished.");

    // This may not be necessary.
    database_sender.send(DatabaseMessage::ShutdownWhenReady).unwrap();
    drop(database_sender);

    let _parse_result = parser_handle.join().unwrap();
    let _database_result = database_handle.join().unwrap();

    Ok(())
}
