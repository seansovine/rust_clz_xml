mod parse;

/// Start of a simple app to read the CLZ books XML file.
///
/// The use of quick-xml was inspired by
///     https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
/// but mostly based on the simple example from the docs,
///     https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html
///
/// A future idea is to load the data into a database. See README for discussion.

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

    let (sender, receiver) = mpsc::channel::<parse::Book>();

    let handle = thread::spawn(move || {
        parse::read_xml(reader, sender)
    });

    // Read books until channel closes, at end of read_xml.
    for book in receiver {
        println!("Found book with title: '{}'", book.title);
    }

    let result = handle.join().unwrap();
    result
}
