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
    pub isbn: String,
    pub authors: Vec<Author>,
}

impl Book {
    pub fn new_option(count: u32) -> Option<Book> {
        Some(Book{ uid: count, title: String::default(), isbn: String::default(), authors: Vec::default() })
    }
}

//

pub struct DatabaseResult{
    pub uid: u32,
    pub message: String,
}

// Message enums named after recipient.

pub enum MainMessage {
    // From parser
    Data(Book),
    WorkComplete,
    // From database
    DatabaseResult(DatabaseResult),
    // Catch-all, for now
    ParserGeneric(String),
}

pub enum DatabaseMessage {
    Data(Book),
    ShutdownWhenReady,
}
