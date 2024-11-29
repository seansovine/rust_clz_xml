use std::sync::mpsc::{Receiver, Sender};

use sqlx::MySqlPool;
use tokio::runtime::Runtime;

use crate::data::{Book, DatabaseMessage, MainMessage};

async fn add_book(book: & Book, pool: & MySqlPool) {
    // Nullable fields can be bound as Options.
    let isbn: Option<&str>;
    if book.isbn.is_empty() {
        isbn = None;
    } else {
        isbn = Some(&book.isbn);
    }

    let book_result =
        sqlx::query("insert into `book` (`title`, `isbn`) values (?, ?)")
        .bind(&book.title).bind(isbn).execute(pool).await;

    let book_id;

    match book_result {
        Err(e) => {
            let error_string = format!("{}", e);
            println!("Error was: {}", error_string);
            // TODO: Send error back to main thread.
            return ()
        }

        Ok(result) => {
            book_id = result.last_insert_id()
        }
    }

    for author in & book.authors {
        let author_result = sqlx::query("insert into `author` (`first_name`, `last_name`) values (?, ?)")
            .bind(&author.first_name).bind(&author.last_name).execute(pool).await;

        let author_id;

        match author_result {
            Err(e) => {
                let error_string = format!("{}", e);
                println!("Error was: {}", error_string);
                // TODO: Send error back to main thread.
                continue
            }

            Ok(result) => {
                author_id = result.last_insert_id()
            }
        }

        let author_book_result = sqlx::query("insert into `author_book` (`author_id`, `book_id`) values (?, ?)")
            .bind(&author_id).bind(&book_id).execute(pool).await;

        match author_book_result {
            Err(e) => {
                let error_string = format!("{}", e);
                println!("Error was: {}", error_string);
                // TODO: Send error back to main thread.
                // TODO: Consider rolling back author insert if this fails.
            }

            Ok(_) => ()
        }
    }
}

pub fn database_main(receiver: Receiver<DatabaseMessage>, sender: Sender<MainMessage>) {
    // Create tokio runtime for blocking on async calls.
    let runtime = Runtime::new().unwrap();

    let user = "mariadb";
    let password = "p@ssw0rd";
    let host = "localhost:3306";
    let database = "collection";

    // Connect to the `collection` database from the `rust_clz_xml` project.
    let connection_string = format!("mysql://{}:{}@{}/{}", user, password, host, database);

    // Create sqlx connection pool.
    let pool_task = MySqlPool::connect(&connection_string);
    let pool = runtime.block_on(pool_task).unwrap();

    // Main loop: Handle messages until main thread closes channel.
    for message in receiver {
        match message {
            DatabaseMessage::Data(data) => {
                // Insert into database.
                runtime.block_on(async {
                    add_book(&data, &pool).await
                });
            }

            _ => {
                // TODO: Handle any other message types.
                ()
            }
        }
    }

    // Gracefully shutdown database connections.
    let pool_close_task = pool.close();
    runtime.block_on(pool_close_task);
}
