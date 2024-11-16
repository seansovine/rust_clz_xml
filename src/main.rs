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
use std::str;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn main() -> std::io::Result<()> {
    // Open XML file.
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;

    // Create buffered reader.
    let mut reader = BufReader::new(file);
    // Create quick-xml reader.
    let mut reader = Reader::from_reader(&mut reader);

    /// Try reading the XML!

    let mut buffer = Vec::new();
    let mut count: u32 = 0;

    // Based on the simple example from the docs. for Reader.
    loop {
        let result = reader.read_event_into(&mut buffer);
        match result {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

            Ok(event) => {
                match event {
                    Event::Eof => break,

                    Event::Start(e) => {
                        let q_name = e.name();
                        let name = str::from_utf8(q_name.as_ref()).unwrap_or_else(|_| {
                            println!("Unable to decode name.");
                            ""
                        });
                        println!("Tag with name: '{}'", name);
                        if name == "book" {
                            count += 1;
                        }
                    }

                    Event::Text(e) => {
                        let text = e.unescape().unwrap().into_owned();
                        println!("Found text: '{}'", text);
                    }

                    Event::End(e) => {
                        let q_name = e.name();
                        let name = str::from_utf8(q_name.as_ref()).unwrap_or_else(|_| {
                            println!("Unable to decode name.");
                            ""
                        });
                        println!("Tag with name: {}", name);
                    }

                    // There are several other `Event`s we do not consider here
                    _ => println!("Event okay but unknown type."),
                }
            }
        }

        // If we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low.
        buffer.clear();
    }

    println!("Found {} 'book' start tags.", count);

    return Ok(());
}
