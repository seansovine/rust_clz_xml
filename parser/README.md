# Design Notes:

This is WIP!

Currently it uses `quick-xml` to parse the XML text input file.
It has a simple state machine to pick out the `<title>` tags
within `<book>` tags. We will soon expand this to pick out other
data we want to extract from book records, starting with authors.

We launch a thread to do the parsing and a thread for the database.
These share a channel to the main thread, and there is a separate
channel from the main thread to the database thread. The idea
is that the parser thread is launched to perform one complete task,
while the database thread stands by and runs any tasks that are
sent to it.

We've implemented basic database insertion for book records using `sqlx`.
We're using `tokio`'s runtime to just block on the async database
methods, but eventually we'll try to take more advantage of the
async nature of `sqlx`.

We also plan to keep trying
out different options for interacting with the
data. One favorite idea is a Golang TUI using the Bubbletea framework,
which we think is nice. We've added a very basic web app using
using Deno to construct HTML from a query to the database. We'd like
to explore some of the nice frameworks for building web apps with
Deno.

## Error / Success Handling in the Rust Code

We've given this some thought now.

Most of the errors in `parse.rs`
that are being checked with `unwrap()` would be programmer or system
errors if they would occur, so we're fine leaving them as panics,
at least while in development. In a production system we would go a
little further and return more easy-to-use information that could be
logged when one of these errors occurs, for use in later debugging.

In `parse.rs` there are a few instances of errors that could happen while
converting between text data types, and it's possible that these could
happen as a result of unexpected text in the file. We will handle these
explicitly.

In `database.rs` there are errors that are more reasonably expected to
occur under normal circumstances, like a database connection that's down
or misconfigured, or a schema update that hasn't been handled right in the code.
And it will be useful to return information back to the main thread when
a successful update has been made here. So here we will
add some more result handling and report errors and successes back to
the main thread.

## Data Management

One major issue is disambiguating the data. It's conceivable to
have two books with the same title or two authors with the same
name. And even more two records of the same book can have
complementary and/or conflicting data.

At some point we will add a utility to help automate disambiguating
and merging corresponding data. There are various ways we could do
this, based on matches in various fields. But this is also the main
reason for the relative lack of unique keys in the schema.
