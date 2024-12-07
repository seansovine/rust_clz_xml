# Bubbletea TUI CLI

This folder has a Golang text user interface (TUI) built using the
Bubbletea framework. So far it provides an interface to the database
services in this project. It allows the user to:

*Reset the database schema from a SQL script:*

This is provides a workflow for updating or trying out updates to the
schema, which is defined in the script `dbutil/scripts/create_db.sql`.

*Empty all data from the database:*

This is useful when testing out different variations of the parser.
Eventually I'd like to implement a system for making and importing dumps
to act as checkpoints for different states of the database.

*Interactively import records from the CLZ export file:*

This allows the user to view book data records found while parsing the
CLZ books XML data file and interactively decide whether to add them
to the database or not. It is work in progress, but the main pieces are
there for a basic version. Longer term I'd like to experiment with
smarter ways to assist the user in finding records that likely correspond
to records already in the database. A first step could be to match on
ISBN number or title, and to automatically skip completely identical
records.

## Program design:

This uses the Bubbletea framework, where the TUI is defined by a
model struct which has `Init`, `View`, and `Update` methods. In particular,
the `Update` method takes in a model and a message and returns an
updated model and a command for the framework to run asynchronously.
The developer defines these methods and any fields on the model that are
used for tracking the application state. Commands are functions that return
a message, and internally Bubbletea runs them using Goroutines and sends
their results to a channel, which will eventually be consumed and
handled internally or passed on to the model's `Update` method.

This particular Bubbletea application defines two models: a main "home"
model that defines the top-level menu of the program and a "data import"
model that launches a Goroutine that calls the parser and reads from
a channel that Goroutine uses to send parse results back. The upshot of
this -- the design and intended use of Bubbletea's framework -- is that
long-running commands are handled asynchrounously so that the TUI the
user interacts with always remains responsive.

I've implemented a basic Rust gRPC service in `parser_grpc` that streams any
book data records it finds back to the caller. I'll soon hook this up to
the TUI, then add another service to allow inserting any records the
user approves via the TUI into the database.
