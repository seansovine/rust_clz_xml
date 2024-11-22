/// Data structures common to multiple modules.

pub struct Author {
    first_name: String,
    middle_name: String,
    last_name: String,
}

pub struct Book {
    pub title: String,
    pub isbn: String,
    authors: Vec<Author>,
}

impl Book {
    pub fn new_option() -> Option<Book> {
        Some(Book{ title: String::default(), isbn: String::default(), authors: Vec::default() })
    }
}

// Message enums named after recipient.

pub enum MainMessage {
    Data(Book),
    WorkComplete,
    Generic(String),
}

pub enum DatabaseMessage {
    Data(Book),
    ShutdownWhenReady,
}
