/// Data structures common to multiple modules.

#[derive(Default)]
pub struct Author {
    pub first_name: String,
    pub last_name: String,
}

pub struct Book {
    // ID unique per run of the parser
    pub uid: u32,

    pub title: String,
    pub isbn: Option<String>,
    pub year: Option<u16>,
    pub publisher: Option<String>,
    pub authors: Vec<Author>,
}

impl Book {
    pub fn new_option(count: u32) -> Option<Book> {
        Some(Book {
            uid: count,
            title: String::default(),
            isbn: None,
            year: None,
            publisher: None,
            authors: Vec::default(),
        })
    }
}

// Database result type.

pub struct DatabaseResult {
    pub uid: u32,
    pub message: String,
}

// Message enums named after recipient.

pub enum MainMessage {
    // From parser
    ParserData(Book),
    ParserWorkComplete,
    // Catch-all, for now
    ParserGeneric(String),

    // From database
    DatabaseResult(DatabaseResult),
}

pub enum DatabaseMessage {
    Data(Book),
    ShutdownWhenReady,
}
