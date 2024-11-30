/// See docs on the quick-xml reader:
///     https://docs.rs/quick-xml/latest/quick_xml/reader/struct.Reader.html
use crate::data::{Author, Book, MainMessage};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use std::io::BufRead;
use std::sync::mpsc::Sender;

static DEBUG_OUT: bool = false;

fn output(message: &str) {
    if DEBUG_OUT {
        println!("{}", message);
    }
}

/// We use a simple state machine to find the title text.
/// The state tracks what tag or section we're currently in.
#[derive(PartialEq, Debug, Copy, Clone)]
enum ParseState {
    BookTag,
    TitleTag,
    IsbnTag,
    CreditTag,
    RoleIdTag,
    // Author info
    AuthorSection,
    //
    AuthorFirstName,
    AuthorLastName,
    // Date info
    PublicationDateSection,
    PublicationYearSection,
    //
    PublicationYearDisplayTag,
    // All tags outside of book
    OtherTag,
}

// NOTE: We assume that the XML is well-formed and has the
// correct structure, and we make no effort here to detect
// and handle badly-formed XML.

fn update_state_on_start(
    state: ParseState,
    bytes: &BytesStart,
    current_book: &mut Option<Book>,
    count: &mut u32,
) -> ParseState {
    let q_name = bytes.name();
    let tag_name = std::str::from_utf8(q_name.as_ref()).unwrap();
    output(&format!("Start tag with name: '{}'", tag_name));

    let new_state = match state {
        ParseState::BookTag => match tag_name {
            "title" => ParseState::TitleTag,

            "isbn" => ParseState::IsbnTag,

            "credit" => ParseState::CreditTag,

            "publicationdate" => ParseState::PublicationDateSection,

            _ => ParseState::BookTag,
        },

        ParseState::CreditTag => match tag_name {
            "roleid" => ParseState::RoleIdTag,

            _ => ParseState::CreditTag,
        },

        ParseState::AuthorSection => match tag_name {
            "firstname" => ParseState::AuthorFirstName,

            "lastname" => ParseState::AuthorLastName,

            _ => ParseState::AuthorSection,
        },

        ParseState::PublicationDateSection => match tag_name {
            "year" => ParseState::PublicationYearSection,

            _ => ParseState::PublicationDateSection,
        }

        ParseState::PublicationYearSection => match tag_name {
            "displayname" => ParseState::PublicationYearDisplayTag,

            _ => ParseState::PublicationYearSection,
        }

        _ => match tag_name {
            "book" => {
                // This ordering means we've chosen 1-based UIDs.
                *count += 1;
                *current_book = Book::new_option(*count);

                ParseState::BookTag
            }

            _ => ParseState::OtherTag,
        },
    };

    new_state
}

fn update_state_on_end(state: ParseState, bytes: &BytesEnd) -> (ParseState, bool) {
    let q_name = bytes.name();
    let tag_name = std::str::from_utf8(q_name.as_ref()).unwrap();
    output(&format!("End tag with name: '{}'", tag_name));

    match state {
        ParseState::BookTag => match tag_name {
            "book" => (ParseState::OtherTag, true),

            _ => (ParseState::BookTag, false),
        },

        ParseState::TitleTag => (ParseState::BookTag, false),

        ParseState::IsbnTag => (ParseState::BookTag, false),

        ParseState::CreditTag => match tag_name {
            "credit" => (ParseState::BookTag, false),

            _ => (ParseState::CreditTag, false),
        },

        ParseState::RoleIdTag => (ParseState::CreditTag, false),

        ParseState::AuthorSection => match tag_name {
            "credit" => (ParseState::BookTag, false),

            _ => (ParseState::AuthorSection, false),
        },

        ParseState::PublicationDateSection => match tag_name {
            "publicationdate" => (ParseState::BookTag, false),

            _ => (ParseState::PublicationDateSection, false),
        }

        _ => (state, false),
    }
}

fn handle_text(state: ParseState, text: &BytesText, current_book: &mut Option<Book>) -> ParseState {
    // NOTE: This allows an early return. The small gain in
    // efficiency it provides might not be worth the effort.
    match state {
        ParseState::TitleTag => {}

        ParseState::IsbnTag => {}

        ParseState::RoleIdTag => {}

        ParseState::AuthorFirstName => {}

        ParseState::AuthorLastName => {}

        ParseState::PublicationYearDisplayTag => {}

        _ => return state,
    }

    let text: String = text.unescape().unwrap().into_owned();

    match state {
        ParseState::TitleTag => {
            output(&format!("Found book with title: '{}'", text));
            current_book.as_mut().unwrap().title = text;
        }

        ParseState::IsbnTag => {
            current_book.as_mut().unwrap().isbn = text;
        }

        ParseState::RoleIdTag => {
            if text == "dfAuthor" {
                current_book
                    .as_mut()
                    .unwrap()
                    .authors
                    .push(Author::default());
                return ParseState::AuthorSection;
            } else {
                return ParseState::CreditTag;
            }
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

        ParseState::PublicationYearDisplayTag => {
            // If parse fails we just set year = None.
            let year: Option<u16> = match text.parse::<u16>() {
                Ok(y) => Some(y),
                Err(_) => None,
            };
            current_book.as_mut().unwrap().year = year;

            // This is all we need from the publication date section.
            return ParseState::PublicationDateSection;
        }

        _ => (),
    }

    state
}

/// Read the XML!
pub fn read_xml<T: BufRead>(
    mut reader: Reader<T>,
    sender: Sender<MainMessage>,
) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    let mut count: u32 = 0;

    let mut parse_state: ParseState = ParseState::OtherTag;
    let mut current_book: Option<Book> = None;

    // Based on the example from the quick-xml docs.
    loop {
        let result = reader.read_event_into(&mut buffer);
        match result {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

            Ok(event) => {
                match event {
                    Event::Eof => break,

                    Event::Start(e) => {
                        parse_state =
                            update_state_on_start(parse_state, &e, &mut current_book, &mut count);
                    }

                    Event::Text(e) => {
                        parse_state = handle_text(parse_state, &e, &mut current_book);
                    }

                    Event::End(e) => {
                        let ready_to_send;
                        (parse_state, ready_to_send) = update_state_on_end(parse_state, &e);

                        if ready_to_send {
                            let message = MainMessage::ParserData(current_book.take().unwrap());
                            sender.send(message).unwrap()
                        }
                    }

                    // Unhandled event types.
                    _ => output("Event okay but unknown type."),
                }
            }
        }

        buffer.clear();
    }

    sender
        .send(MainMessage::ParserGeneric(format!(
            "Found {} 'book' start tags.",
            count
        )))
        .unwrap();

    sender.send(MainMessage::ParserWorkComplete).unwrap();

    Ok(())
}
