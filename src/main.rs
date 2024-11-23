mod parse;
mod database;
mod data;

/// Start of an app to read the CLZ books XML file.
///
/// The use of quick-xml was inspired by
///     https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
/// but mostly based on the simple example from the docs,
///     https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html
///
/// Loads book data extracted from the XML file into a database.
/// See README's for further discussion.

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
    // Start parser thread.
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

    // Read books until parser channel sends WorkComplete.
    for message in main_receiver {
        match message {
            MainMessage::Data(book) => {
                println!("Found book with title: '{}'", book.title);

                // Send the book data to the database thread.
                database_sender.send(DatabaseMessage::Data(book)).unwrap()
            }

            MainMessage::WorkComplete => break,

            MainMessage::Generic(message) => {
                println!("{}", message);
            }
        }
    }

    println!("Finished.");

    // (This may not be necessary.)
    database_sender.send(DatabaseMessage::ShutdownWhenReady).unwrap();
    // Close channel to database.
    drop(database_sender);

    let _parse_result = parser_handle.join().unwrap();
    let _database_result = database_handle.join().unwrap();

    Ok(())
}
