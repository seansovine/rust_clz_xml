use tokio::sync::mpsc::{Receiver, Sender};

use sqlx::MySqlPool;
use tokio::runtime::Runtime;

use crate::data::{Book, DatabaseMessage, DatabaseResult, MainMessage};

async fn add_book(book: &Book, pool: &MySqlPool) -> Result<String, String> {
    let book_result = sqlx::query(
        "insert into `book` (`title`, `isbn`, `year`, `publisher`) values (?, ?, ?, ?)",
    )
    .bind(&book.title)
    .bind(&book.isbn)
    .bind(&book.year)
    .bind(&book.publisher)
    .execute(pool)
    .await;

    let book_id;

    // Keeping it simple for now, we return a string on success or failure.
    let result_message: String;

    match book_result {
        Err(e) => {
            result_message = format!(
                "Failed to insert book with title '{}'\n  Database error: {}",
                &book.title, e
            );
            return Err(result_message);
        }

        Ok(result) => {
            book_id = result.last_insert_id();
            result_message = format!("Inserted book with id {}.", book_id)
        }
    }

    for author in &book.authors {
        let author_result =
            sqlx::query("insert into `author` (`first_name`, `last_name`) values (?, ?)")
                .bind(&author.first_name)
                .bind(&author.last_name)
                .execute(pool)
                .await;

        let author_id;

        match author_result {
            Err(e) => {
                let error_string = format!("{}", e);
                println!("Error was: {}", error_string);
                // TODO: Send error back to main thread.
                continue;
            }

            Ok(result) => author_id = result.last_insert_id(),
        }

        let author_book_result =
            sqlx::query("insert into `author_book` (`author_id`, `book_id`) values (?, ?)")
                .bind(&author_id)
                .bind(&book_id)
                .execute(pool)
                .await;

        match author_book_result {
            Err(e) => {
                let error_string = format!("{}", e);
                println!("Error was: {}", error_string);
                // TODO: Send error back to main thread.
                // TODO: Consider rolling back author insert if this fails.
            }

            Ok(_) => (),
        }
    }

    Ok(result_message)
}

pub fn database_main(mut receiver: Receiver<DatabaseMessage>, sender: Sender<MainMessage>) {
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
    while let Some(message) = receiver.blocking_recv() {
        match message {
            DatabaseMessage::Data(data) => {
                // Insert into database.
                let result = runtime.block_on(async { add_book(&data, &pool).await });

                let message = result.unwrap_or_else(|message| message);
                sender
                    .blocking_send(MainMessage::DatabaseResult(DatabaseResult {
                        uid: data.uid,
                        message,
                    }))
                    .unwrap()
            }

            _ => (), // TODO: Handle any other message types.
        }
    }

    // Gracefully shutdown database connections.
    let pool_close_task = pool.close();
    runtime.block_on(pool_close_task);
}
