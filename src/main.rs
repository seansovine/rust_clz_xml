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
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::thread;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

static DEBUG_OUT: bool = false;

fn output(message: &str) {
    if DEBUG_OUT {
        println!("{}", message);
    }
}

/// We use a simple state machine to find the title text.
#[derive(PartialEq, Debug)]
enum ParseState {
    BookTag,
    TitleTag,
    OtherTag
}

fn update_state_on_start(state: ParseState, tag_name: &str) -> ParseState {
    match state {
        ParseState::BookTag => match tag_name {
            "title" => ParseState::TitleTag,
            _ => ParseState::BookTag
        }

        _ => match tag_name {
            "book" => ParseState::BookTag,
            _ => ParseState::OtherTag
        }
    }
}

fn update_state_on_end(state: ParseState, tag_name: &str) -> ParseState {
    match state {
        ParseState::TitleTag => ParseState::BookTag,

        ParseState::BookTag => match tag_name {
            "book" => ParseState::OtherTag,
            _ => ParseState::BookTag
        }
        _ => state
    }
}

/// Read the XML!
fn read_xml<T: BufRead>(mut reader: Reader<T>) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    let mut count: u32 = 0;

    let mut parse_state: ParseState = ParseState::OtherTag;

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
                        let name = str::from_utf8(q_name.as_ref()).unwrap();
                        output(&format!("Start tag with name: '{}'", name));
                        if name == "book" {
                            count += 1;
                        }

                        parse_state = update_state_on_start(parse_state, name);
                    }

                    Event::Text(e) => {
                        if parse_state == ParseState::TitleTag {
                            let text = e.unescape().unwrap().into_owned();
                            println!("Found book with title: '{}'", text);
                        }
                    }

                    Event::End(e) => {
                        let q_name = e.name();
                        let name = str::from_utf8(q_name.as_ref()).unwrap();
                        output(&format!("End tag with name: '{}'", name));

                        parse_state = update_state_on_end(parse_state, name);
                    }

                    // There are several other `Event`s we do not consider here
                    _ => output("Event okay but unknown type."),
                }
            }
        }

        // If we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low.
        buffer.clear();
    }

    println!("Found {} 'book' start tags.", count);

    return Ok(());
}

fn main() -> std::io::Result<()> {
    // Open XML file.
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;

    // Create buffered reader.
    let reader = BufReader::new(file);
    // Create quick-xml reader.
    let reader = Reader::from_reader(reader);

    let handle = thread::spawn(move || {
        read_xml(reader)
    });

    let result = handle.join().unwrap();
    result
}
