/// Async task to add book data received over a channel to the database.
///
use crate::data::{Book, DatabaseMessage, DatabaseResult, MainMessage};

use std::collections::VecDeque;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::{JoinError, JoinSet};

/// Add book and its corresponding author data to database.
async fn add_book(book: Book, pool: MySqlPool) -> (Result<String, String>, Book) {
    let book_result = sqlx::query(
        "insert into `book` (`title`, `isbn`, `year`, `publisher`) values (?, ?, ?, ?)",
    )
    .bind(&book.title)
    .bind(&book.isbn)
    .bind(&book.year)
    .bind(&book.publisher)
    .execute(&pool)
    .await;

    // Keeping it simple for now, we return a string on success or failure.
    let mut result_message: String;

    let book_id;

    match book_result {
        Err(e) => {
            result_message = format!(
                "Failed to insert book with title '{}'\n  Database error: {}",
                &book.title, e
            );
            return (Err(result_message), book);
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
                .execute(&pool)
                .await;

        let author_id;

        match author_result {
            Err(e) => {
                let error_string = format!("{}", e);
                result_message = format!("{result_message} With error: {error_string}");
                continue;
            }

            Ok(result) => author_id = result.last_insert_id(),
        }

        let author_book_result =
            sqlx::query("insert into `author_book` (`author_id`, `book_id`) values (?, ?)")
                .bind(&author_id)
                .bind(&book_id)
                .execute(&pool)
                .await;

        match author_book_result {
            Err(e) => {
                let error_string = format!("{}", e);
                result_message = format!("{result_message} With error: {error_string}");
                // TODO: Consider rolling back author insert if this fails.
            }

            Ok(_) => (),
        }
    }

    (Ok(result_message), book)
}

// -----------------------------
// Top-level task main function.

pub async fn database_main(mut receiver: Receiver<DatabaseMessage>, sender: Sender<MainMessage>) {
    let user = "mariadb";
    let password = "p@ssw0rd";
    let host = "localhost:3306";
    let database = "collection";

    // Maximum allowed concurrent database tasks,
    // and minimum number of database connections.
    const MAX_TASKS: u32 = 10;

    // Connect to the `collection` database from the `rust_clz_xml` project.
    let connection_string = format!("mysql://{}:{}@{}/{}", user, password, host, database);

    // Options for connection pool.
    let pool_options = MySqlPoolOptions::new().min_connections(MAX_TASKS);

    // Create sqlx connection pool.
    let pool_task = pool_options.connect(&connection_string);
    let pool = pool_task.await.unwrap();

    let mut join_set = JoinSet::new();
    let mut queue: VecDeque<Book> = VecDeque::new();

    let mut num_tasks: u32 = 0;
    let mut ready_to_shutdown: bool = false;

    // Main loop: Handle messages and spawn database tasks until
    // main thread drops channel and all tasks are done.
    loop {
        while num_tasks < MAX_TASKS && !queue.is_empty() {
            // Make add to db task.
            let book_data = queue.pop_front().unwrap();

            // NOTE: We clone because the task future must hold onto the connection
            // pool until it is complete. The docs say that a cloned pool refers to
            // the same underlying pool of connections, so this should be okay. But
            // perhaps there is a more efficient way to handle it.
            let add_task = add_book(book_data, pool.clone());
            num_tasks += 1;

            join_set.spawn(add_task);
        }

        tokio::select! {
            val = receiver.recv() => {
                handle_message(val, &mut queue, &mut ready_to_shutdown);
            }
            val = join_set.join_next() => {
                handle_result(val, &mut num_tasks, &sender).await;
            }
        }

        if ready_to_shutdown && join_set.is_empty() && queue.is_empty() {
            break;
        }
    }

    // Gracefully shutdown database connections.
    pool.close().await;
}

// ---------------------------------------------------
// Helpers for receiving messages and sending results.

/// Check channel receiver and queue up book data received for insert.
fn handle_message(
    msg_opt: Option<DatabaseMessage>,
    queue: &mut VecDeque<Book>,
    ready_to_shutdown: &mut bool,
) {
    match msg_opt {
        Some(message) => {
            match message {
                DatabaseMessage::Data(data) => {
                    let book_data = data;
                    queue.push_back(book_data);
                }

                // Currently not expecting any other message types.
                _ => panic!("Unexpected message received."),
            }
        }

        None => *ready_to_shutdown = true,
    }
}

/// Handle results when a database add task completes and notify main thread.
async fn handle_result(
    val: Option<Result<(Result<String, String>, Book), JoinError>>,
    num_tasks: &mut u32,
    sender: &Sender<MainMessage>,
) {
    match val {
        Some(result) => {
            let (result, book_data) = result.unwrap();
            let message = result.unwrap_or_else(|msg| msg);
            let database_result = DatabaseResult {
                uid: book_data.uid,
                message,
            };

            *num_tasks -= 1;

            // Notify main thread that task is complete, with success and/or error message.
            sender
                .send(MainMessage::DatabaseResult(database_result))
                .await
                .unwrap()
        }

        // None indicates no tasks in the join set.
        None => {}
    }
}
