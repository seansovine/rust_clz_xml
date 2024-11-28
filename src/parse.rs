use crate::data::{Author, Book, MainMessage};

use std::io::BufRead;
use std::sync::mpsc::Sender;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;

static DEBUG_OUT: bool = false;

fn output(message: &str) {
    if DEBUG_OUT {
        println!("{}", message);
    }
}

/// We use a simple state machine to find the title text.
#[derive(PartialEq, Debug, Copy, Clone)]
enum ParseState {
    BookTag,
    TitleTag,
    IsbnTag,
    CreditTag,
    RoleIdTag,
    OtherTag,
    //
    AuthorSection,
    //
    AuthorFirstName,
    AuthorLastName,
}

fn update_state_on_start(state: ParseState, bytes: & BytesStart, current_book: & mut Option<Book>) -> (ParseState, String) {
    let q_name = bytes.name();
    let tag_name = std::str::from_utf8(q_name.as_ref()).unwrap();
    output(&format!("Start tag with name: '{}'", tag_name));

    let new_state = match state {
        ParseState::BookTag => match tag_name {
            "title" => ParseState::TitleTag,

            "isbn" => ParseState::IsbnTag,

            "credit" => ParseState::CreditTag,

            _ => ParseState::BookTag
        }

        ParseState::CreditTag => match tag_name {
            "roleid" => {
                ParseState::RoleIdTag
            },

            _ => ParseState::CreditTag
        }

        ParseState::AuthorSection => match tag_name {
            "firstname" => ParseState::AuthorFirstName,

            "lastname" => ParseState::AuthorLastName,

            _ => ParseState::AuthorSection
        }

        _ => match tag_name {
            "book" => {
                *current_book = Book::new_option();
                ParseState::BookTag
            }

            _ => ParseState::OtherTag
        }
    };

    (new_state, String::from(tag_name))
}

fn update_state_on_end(state: ParseState, bytes: & BytesEnd) -> (ParseState, bool) {
    let q_name = bytes.name();
    let tag_name = std::str::from_utf8(q_name.as_ref()).unwrap();
    output(&format!("End tag with name: '{}'", tag_name));

    match state {
        ParseState::TitleTag => (ParseState::BookTag, false),

        ParseState::IsbnTag => (ParseState::BookTag, false),

        ParseState::CreditTag => match tag_name {
            "credit" => (ParseState::BookTag, false),

            _ => (ParseState::CreditTag, false),
        }

        ParseState::RoleIdTag => (ParseState::CreditTag, false),

        ParseState::BookTag => match tag_name {
            "book" => (ParseState::OtherTag, true),

            _ => (ParseState::BookTag, false)
        }

        ParseState::AuthorSection => match tag_name {
            "credit" => {
                (ParseState::BookTag, false)
            },

            _ => (ParseState::AuthorSection, false)
        }

        _ => (state, false)
    }
}

fn handle_text(state: ParseState, text: & BytesText, current_book: &mut Option<Book>) -> ParseState {
    match state {
        ParseState::TitleTag => {}

        ParseState::IsbnTag => {}

        ParseState::RoleIdTag => {}

        ParseState::AuthorFirstName => {}

        ParseState::AuthorLastName => {}

        _ => return state
    }

    let text = text.unescape().unwrap().into_owned();

    match state {
        ParseState::TitleTag => {
            output(&format!("Found book with title: '{}'", text));
            current_book.as_mut().unwrap().title = text;

        }

        ParseState::IsbnTag => {
            current_book.as_mut().unwrap().isbn = text;
        }

        ParseState::RoleIdTag => if text == "dfAuthor" {
            current_book.as_mut().unwrap().authors.push(Author::default());
            return ParseState::AuthorSection;
        } else {
            return ParseState::CreditTag;
        }

        ParseState::AuthorFirstName => {
            let new_author = current_book.as_mut().unwrap().authors.last_mut().unwrap();
            new_author.first_name = text;

            return ParseState::AuthorSection;
        }

        ParseState::AuthorLastName => {
            let new_author = current_book.as_mut().unwrap().authors.last_mut().unwrap();
            new_author.last_name = text;

            return ParseState::AuthorSection;
        }

        _ => ()
    }

    state
}

/// Read the XML!
pub fn read_xml<T: BufRead>(mut reader: Reader<T>, sender: Sender<MainMessage>) -> std::io::Result<()> {
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
                        let name: String;
                        (parse_state, name) = update_state_on_start(parse_state, &e, &mut current_book);

                        if name == "book" {
                            count += 1;
                        }
                    }

                    Event::Text(e) => {
                        parse_state = handle_text(parse_state, &e, &mut current_book);
                    }

                    Event::End(e) => {
                        let ready_to_send;
                        (parse_state, ready_to_send) = update_state_on_end(parse_state, &e);

                        if ready_to_send {
                            let message = MainMessage::Data(current_book.take().unwrap());
                            sender.send(message).unwrap()
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

    sender.send(MainMessage::Generic(format!("Found {} 'book' start tags.", count))).unwrap();

    sender.send(MainMessage::WorkComplete).unwrap();

    return Ok(());
}
