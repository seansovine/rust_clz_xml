use std::sync::mpsc::{Receiver, Sender};

use sqlx::MySqlPool;
use tokio::runtime::Runtime;

use crate::data::{Book, DatabaseMessage, MainMessage};

async fn add_book(book: & Book, pool: & MySqlPool) {
    let result = sqlx::query("insert into `book` (`title`) values (?)")
        .bind(book.title.clone()).execute(pool).await;

    match result {
        Err(e) => {
            let error_string = format!("{}", e);
            println!("Error was: {}", error_string);
            // TODO: Send error back to main thread.
            ()
        }

        Ok(_) => ()
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
}
