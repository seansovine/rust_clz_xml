/// An app to read the CLZ books XML file.
/// Loads book data extracted from the XML file into a database.
/// See README files for further discussion.
use clz_data::data::{DatabaseMessage, DatabaseResult, MainMessage};
use clz_data::database;
use clz_data::parse;

use std::collections::HashSet;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::thread;

use tokio::sync::mpsc;

use colored::Colorize;
use quick_xml::reader::Reader;
use tokio::runtime::Runtime;

const CHANNEL_BUFFER_SIZE: usize = 1000;

fn main() -> std::io::Result<()> {
    // Open XML file.
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", "Usage: ./clz-data <file>".red());

        return Err(Error::new(ErrorKind::Other, "bad arguments"));
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;

    // Create buffered reader.
    let reader = BufReader::new(file);
    // Create quick-xml reader.
    let reader = Reader::from_reader(reader);

    // Channel from workers to main.
    let (main_sender, mut main_receiver) = mpsc::channel::<MainMessage>(CHANNEL_BUFFER_SIZE);

    let main_sender_parser = main_sender.clone();
    // Start parser thread.
    let parser_handle = thread::spawn(move || parse::read_xml(reader, main_sender_parser));

    // Create channel to database thread.
    let (database_sender, database_receiver) =
        mpsc::channel::<DatabaseMessage>(CHANNEL_BUFFER_SIZE);
    // Rename main sender for symmetry ;)
    let main_sender_database = main_sender;
    // Start database thread.
    let database_handle = thread::spawn(move || {
        // Give the database thread its own runtime to
        // poll the futures of its asynchronous tasks.
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            database::database_main(database_receiver, main_sender_database).await
        });
    });

    let mut database_tasks = HashSet::new();

    let parser_tag = "PARSER".yellow();
    let database_tag = "DATABASE".red();

    // Read books until parser channel sends WorkComplete.
    while let Some(message) = main_receiver.blocking_recv() {
        match message {
            MainMessage::ParserData(book) => {
                println!(
                    ">> {parser_tag}: UID {}: Found book with title: '{}'",
                    book.uid, book.title
                );

                database_tasks.insert(book.uid);
                // Send the book data to the database thread.
                database_sender
                    .blocking_send(DatabaseMessage::Data(book))
                    .unwrap()
            }

            MainMessage::ParserWorkComplete => {
                println!("\n -- {} --\n", "Parser Finished.".green());
            }

            MainMessage::DatabaseResult(DatabaseResult { uid, message }) => {
                println!("<< {database_tag}: Result for UID {}: {}", uid, message);

                database_tasks.remove(&uid);

                if database_tasks.is_empty() {
                    // TODO: Use something like curses to keep these at bottom of terminal.
                    println!("\n -- {} --\n", "All database tasks complete.".blue());

                    break;
                }
            }

            MainMessage::ParserGeneric(message) => {
                println!(">> {parser_tag}: {}", message);
            }
        }
    }

    // (This may not be necessary.)
    database_sender
        .blocking_send(DatabaseMessage::ShutdownWhenReady)
        .unwrap();
    // Close channel to database.
    drop(database_sender);

    let _parse_result = parser_handle.join().unwrap();
    let _database_result = database_handle.join().unwrap();

    println!(" --> {} <--", "Done.".green());

    Ok(())
}
