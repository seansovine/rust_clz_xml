use std::sync::mpsc::{Receiver, Sender};

use crate::data::{DatabaseMessage, MainMessage};

pub fn database_main(receiver: Receiver<DatabaseMessage>, sender: Sender<MainMessage>) {
    // TODO: Add main loop to handle messages
    //       and functions to run database queries
    //       using sqlx.

    for message in receiver {
        match message {
            DatabaseMessage::Data(data) => {
                // TODO: Insert into database.
            }

            _ => {
                // TODO: Handle other message types.
                ()
            }
        }
    }
}
