use std::io::BufRead;
use std::sync::mpsc::Sender;
use quick_xml::events::Event;
use quick_xml::Reader;

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

fn update_state_on_start(state: ParseState, tag_name: &str, current_book: & mut Option<Book>) -> ParseState {
    match state {
        ParseState::BookTag => match tag_name {
            "title" => ParseState::TitleTag,

            _ => ParseState::BookTag
        }

        _ => match tag_name {
            "book" => {
                *current_book = Book::new_option();
                ParseState::BookTag
            }

            _ => ParseState::OtherTag
        }
    }
}

fn update_state_on_end(state: ParseState, tag_name: &str) -> (ParseState, bool) {
    match state {
        ParseState::TitleTag => (ParseState::BookTag, false),

        ParseState::BookTag => match tag_name {
            "book" => (ParseState::OtherTag, true),

            _ => (ParseState::BookTag, false)
        }

        _ => (state, false)
    }
}

struct Author {
    first_name: String,
    middle_name: String,
    last_name: String,
}

pub struct Book {
    pub title: String,
    authors: Vec<Author>,
}

impl Book {
    pub fn new_option() -> Option<Book> {
        Some(Book{ title: String::default(), authors: Vec::default() })
    }
}

/// Read the XML!
pub fn read_xml<T: BufRead>(mut reader: Reader<T>, sender: Sender<Book>) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    let mut count: u32 = 0;

    let mut parse_state: ParseState = ParseState::OtherTag;
    let mut current_book: Option<Book> = None;

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
                        let name = std::str::from_utf8(q_name.as_ref()).unwrap();
                        output(&format!("Start tag with name: '{}'", name));
                        if name == "book" {
                            count += 1;
                        }

                        parse_state = update_state_on_start(parse_state, name, &mut current_book);
                    }

                    Event::Text(e) => {
                        if parse_state == ParseState::TitleTag {
                            let text = e.unescape().unwrap().into_owned();
                            output(&format!("Found book with title: '{}'", text));

                            current_book.as_mut().unwrap().title = text;
                        }
                    }

                    Event::End(e) => {
                        let q_name = e.name();
                        let name = std::str::from_utf8(q_name.as_ref()).unwrap();
                        output(&format!("End tag with name: '{}'", name));

                        let ready_to_send;
                        (parse_state, ready_to_send) = update_state_on_end(parse_state, name);

                        if ready_to_send {
                            sender.send(current_book.take().unwrap()).unwrap()
                        }
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